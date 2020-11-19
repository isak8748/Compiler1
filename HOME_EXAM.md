# Home Exam D7050E


## Your repo

https://github.com/isak8748/Compiler1

## Your syntax


Program:

```
: ("fn" Function)*
;
```

Function:

```
: Id "(" Params ","* ")" ("->" Id) "{" Body "}"
;
```

Params:
```
:(Id TypeSpec ",")* [Id TypeSpec]
;
```

Body:
```
:(Instruction ";")* [Instruction]
;
```

Id:
```
:([a-z]|[A-Z]) ([a-z]|[A-Z]|[0-9]|_)*
;
```

Num:
```
:[0-9]+
;
```

Term:
```
:Num
|Id
|"(" BoolExp ")"
|FunctionCall
|UnaryOp Term
|"true"
|"false"
;
```

Factor:
```
:Factor FactorOp Term
|Term
;
```

Expr:

```
:Expr ExprOp Factor
|Factor
;
```

Comparison:

```
:Comparison CompareOp Expr
|Expr
;
```

BoolExp:
```
:BoolExp BooleanOp Comparison
|Comparison
;
```

FactorOp:
```
:"*"
|"/"
|"%"
;
```

ExprOp:
```
:"+"
|"-"
;
```
CompareOp:
```
:"<"
|">"
|"<="
|">="
|"=="
|"!="
;
```

BooleanOp:
```
:"&&"
;"||"
;
```

UnaryOp:
```
:"!"
|"-"
|"&"
|"&mut"
|"*"
;
```


Instruction:
```
:BoolExp
|Declaration
|Stmt
|If
|IfElse
|While
|Return
|WriteByReference
;
```

Return:
```
:"return" [BoolExp]
;
```

Stmt:
```
:Id "=" Instruction
;
```

Declaration:
```
:"let" "mut" Id [TypeSpec] ["=" Instruction]
|"let" Id [TypeSpec] ["=" Instruction]
;
```

WriteByReference:
```
:UnaryOp Id "=" Instruction
;
```

FunctionCall:
```
:Id "(" Arguments ")"
;
```

Arguments:
```
:(BoolExp ",")* [BoolExp]
;
```

If:
```
:"if" BoolExp "{" Body "}"
;
```

IfElse
```
:"if" BoolExp "{" Body "}" "else" "{" Body "}"
;
```

While:
```
:"while" BoolExp "{" Body "}"
;
```

TypeSpec:
```
:": i32"
|": bool"
|": &i32"
|": &mut i32"
|": &bool"
|": &mut bool"
;
```

Parethesized sub expressions are supported as well as operator precedence. "*", "/" and "%" have the highes precedence, then "+" and "-", then comparisons and lastly "||" and "&&". I have worked alone on this project.

Example Program
```rust
fn math(x: i32, y: i32) -> bool{
    let a: i32 = 13 * (2 + 1);
    let b = true;
    let mut c = true;
    if(x % 2 >= 5){
        c = false;
    }
    else{
        c = true;
    };
    return c && b;
}

fn foo(b: bool, c: bool) -> bool{
    let a = &mut b;
    if *a && c{
        *a = false;
    };
    return *a;
}

fn main() -> i32{
    let j = 13;
    let d = &j;
    let i = math(56, j);
    let mut x = 0;
    if foo(true, true){
        x = 100;
    }
    else{
        x = 60;
    };
    return x;
}
```
The above program showcases the grammar. 

Note the semicolons after all if and if/else statements. These are required by the parser unless these statements are the last in a block of instructions:

```rust
fn foo(){
    if true{
        let x = false;
    }
    return;
}
```
This function would not be accepted by the grammar.





## Your semantics


Constants:

<img src="https://render.githubusercontent.com/render/math?math=\frac{}{<n, σ> -> n}">

```math
\frac{}{<n, σ> -> n}
```
Same for booleans

Variables:
```math
\frac{}{<x, σ> -> σ(x)}
```

Arithmetic operations:
```math
\frac{}{<n1 ⊕ n2, σ> -> n3}
```
It is similar for the boolean operations. For comparison operators the result will always be a boolean value. All boolean operations can be used on numbers and 2 of them on booleans.

Sub-expressions:
```math
\frac{<a1, σ> -> a1'}{<a1 ⊕ a2, σ> -> <a1' ⊕ a2, σ>}
```
Parentheses can be used to create sub expressions. Otherwise operator precedence determines the order sub expressions are evaluated.

Assignment:
```math
\frac{<a1, σ> -> a1'}{<x := n, σ> -> <skip, σ[x->n]>}
```
σ[x->n] is the updated variable environment. Declaration of variables work similarly. If an old variable with the same name exists it will be replaced.

Sequence:

```math
\frac{<c0, σ> -> <c0', σ'>}{<c0;c1, σ> -> <c0';c1, σ'>}
```

If/else:
```math
\frac{<b, σ> -> b'}{<if b then c1 else c2, σ> -> <if b' then c1 else c2>}
```
The condition will eventually evaluate to true or false(ensured  by the typechecker).


While:
```math
\frac{}{<while b do c, σ> -> <if b then (c; while b do c) else skip, σ>}
```




## Your type checker


Arithmetic operations:
```math
\frac{}{<i32 ⊕ i32, σ> -> i32}
```
For logical operations i32 is replaced with boolean. For comparisons the result is boolean and the operands can be either boolean or i32 (depending on the operation). The typechecker will make sure the operands are the correct type.

Correct operation expressions:
```rust
12 + 323
1 * 12 + 34
13 % (1 +2)
true || false
true == false
13 < 12
23 >= 12
```

Incorrect expressions:
```rust
5 * true
5 || 6
true && 45
false + true
true < false
```

Unary operations:
Unary - requires the operand to be a number, ! requires the operand to be a boolean. * requires the operand to be a reference. & and &mut require the operand to be an identifier.

Correct program:
```rust
let a: i32 = -5;
let b: bool = !true;
let c = &a;
let d = &mut b;
let x = *c;
```

Incorrect program:
```rust
let a = -true;
let b = !(5 + 4);
let c = *a;
let d = &true;
let x = &mut 5;
```

There are a few ways to declare a variable:
```rust
let a: i32 = 1;
let b = bool;
let c;
let d: bool;
c = 123;
d = true;
```

As seen the type can be specified. The typechecker will then validate that the expression is of the correct type. If the type is not specified the typechecker will infer the type from the expression. If no value is assigned at all the variable is added to context and if no type is specified it will remain of unknown type until assigned a value.

Incorrect declarations:
```rust
let a: i32 = true;
let b: bool = 5;
```

When assigning to a variable its type is checked in the context and the typechecker makes sure it matches the expression assigned. The typechecker also checks if the variable is mutable.

Incorrect assignments:

```rust
let mut a = 5;
a = true;
let x = true;
x = false;
```

If/Else, If and While:

Correct program:
```rust
let b: bool = true;
if b || true && 3 < 5{
    let x = 3;
}
else{
    let x = 12;
}
```
Incorrect: program:
```rust
while 5 +13{
    foo(x);
}
```

The condition is evaluated to make sure it is of boolean type. For every instruction in all branches the typecheker checks that it obeys all type checking rules.

For If/Else if the last instruction in both blocks is an expression with no semicolon the type of the If/Else instruction will be the type of these expressions to allow assigning to a variable:
```rust
let a = if true{5} else {6};
```

Functions:
The specified return type of every function is made sure to be correct by the typechecker.
The return type and the type of every parameter is inserted into context.

Incorrect function:
```rust
fn fib(x: i32) -> i32{
    return true;
}
```
Correct funtion:
```rust
fn square(x: i32, bool: b) -> i32{
    if b{
        x*x
    }
    else{
        -1
    }
}
```
Here the return type will be inserted as i32 and the parameters will be [i32, bool].


Function calls:
The type of the function call is the return type of the function (read from context).
Every argument in the call is compared to the parameter types to make sure its correct.

Incorrect calls:
```rust
let a = square(true, true); //Incorrect argument types
let b: bool = square(13, true); //Return type not boolean
```

Correct call:
```rust
let a: i32 = square(5, true);
```





## Your borrrow checker


Below is an example of well formed borrows:

```rust
let a = 2;
let b = &a;
let c = &mut a;
*c = 10;
let x = &a;
let y = &a;
a = 6;
let d = *y;
```
When c is created b is removed. c can be used until either a new borrow is created or the value of a gets changed directly(both happen here). The implementation uses a stack of borrows for each variable. When a mutable reference is created the stack is cleared. Non mutable references will just be added to the stack. So if a mutable reference exists it will always be one at the bottom of the stack. When assigning directly to a variable this is removed and the the mutable reference can not be accessed when non-mutable references also exist. This ensures every variable can only have one unique (usable) mutable references or any number of non-mutable references withing a "block" of code.

Below is an example of ill formed borrows:
```rust
let a = 2;
let b = &mut a;
a = 13;
*b = 12;
```
Within the block there are two ways to write to the variable a: directly and with the reference

Another example:
```rust
let a = 2;
let b = &mut a;
let c = &a;
let x = *b;
```
Here b will be unreachable as there would otherwise be both a mutable and non-mutable reference to a in a block of code.


## Overall course goals and learning outcomes.

Comment on the alignment of the concrete course goals (taken from the course description) to the theory presented, work You have done and knowledge You have gained. (I have put some comments in [...]).

- Lexical analysis, syntax analysis, and translation into abstract syntax.

I have learned a lot about this through building my AST. I think lalrpop worked really well to relatively quickly create a parser allthough it was a challenge at first. 

- Regular expressions and grammars, context-free languages and grammars, lexer and parser generators. [lalr-pop is a classical parser generator, it auto generated the lexer for you based on regular expressions but allows for you to define the lexer yourself for more control]

I have learned to use regular expressions when creating my grammar. I learned about context-free languages and grammars in class and lalrpop was used as parser and lexer generator.

- Identifier handling and symbol table organization. Type-checking, logical inference systems. [SOS is a logical inference system]

Creating the data structures needed to store information for functions and variables was also a nice challenge and i had to redesign these a few times. I also learned a lot from doing the typechecking. This was probably the most challenging and time consuming part of the program. 

- Intermediate representations and transformations for different languages. [If you attended, Recall lectures relating LLVM/Crane-lift, discussions on SSA (Single Static Assignment) used in LLVM/Crane-lift, and discussions/examples on high [level optimization](https://gitlab.henriktjader.com/pln/d7050e_2020/-/tree/generics_and_traits/examples)]

I learned a bit about this during the lectures but did not implement it.

- Code optimization and register allocation. Machine code generation for common architectures. [Both LLVM/Crane-Lift does the "dirty work" of backend optimization/register allocation leveraging the SSA form of the LLVM-IR]

Also learned  a bit during the lectures but did not icorporate in my program.


Comment on additional things that you have experienced and learned throughout the course.
