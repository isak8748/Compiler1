# Home Exam D7050E

- Fork this repo and put your answers (or links to answers) in THIS file.

## Your repo

- Link to your repo here:

## Your syntax

- Give an as complete as possible EBNF grammar for your language.

- Give an example that showcases all rules of your EBNF. The program should "do" something as used in the next exercise.

- For your implementation, show that your compiler successfully accepts the input program.

- Give a set of examples that are syntactically illegal, and rejected by your compiler.

- Compare your solution to the requirements (as stated in the README.md). What are your contributions to the implementation.

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

Example Program
```rust
fn math(x: i32, y: i32) -> bool{
    let a: i32 = 13 * (2 + 1);
    let b = true;
    let c;
    if(x % 2 >= 5){
        c = false;
    }
    else{
        c = true;
    }
    return c && b;
}

fn main() -> i32{

}
```


```math
\frac{12}{12}
```

Parethesized sub expressions are supported as well as operator precedence. "*", "/" and "%" have the highes precedence, then "+" and "-", then comparisons and lastly "||" and "&&". I have worked alone on this project.


## Your semantics

- Give a (simplified) Structural Operational Semantics (SOS) for your language. You don't need to detail rules that are similar (follow the same pattern). Regarding variable environment (store) you may omit details as long as the presentation is easy to follow.

- Explain (in text) what an interpretation of your example should produce, do that by dry running your given example step by step. Relate back to the SOS rules. You may skip repetitions to avoid cluttering.

- For your implementation, give a program (or set of test programs) that cover all the semantics of your language that you have successfully implemented. (Maybe a subset of the input language accepted by the grammar.)

- Compare your solution to the requirements (as stated in the README.md). What are your contributions to the implementation.

Constants:

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

- Give a simplified set of Type Checking Rules for your language (those rules look very much like the SOS rules, but over types not values). Also here you don't need to detail rules that are similar (follow the same pattern).

- Demonstrate each "type rule" by an example. You may use one or several "programs" to showcase where rules successfully apply.

- For your implementation, give a set of programs demonstrating that ill-typed programs are rejected, connect back to the Type Checking Rules to argue why these are illegal and thus should be rejected.

- Compare your solution to the requirements (as stated in the README.md). What are your contributions to the implementation.

Arithmetic operations:
```math
\frac{}{<i32 ⊕ i32, σ> -> i32}
```
For logical operations i32 is replaced with boolean. For comparisons the result is boolean and the operands can be either boolean or i32 (depending on the operation). The typechecker will make sure the operands are the correct type.

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
if b || a && 3 < 5{
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

- Give a specification for well versus ill formed borrows. (What are the rules the borrow checker should check).

- Demonstrate the cases of ill formed borrows that your borrow checker is able to detect and reject.

- Compare your solution to the requirements (as stated in the README.md). What are your contributions to the implementation.

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

## Your LLVM/Crane-Lift backend (optional)

- Let your backend produce LLVM-IR/Crane Lift IR for an example program (covering the translations implemented).

- Describe the translation process, and connect back to the generated IR.

- Compare your solution to the requirements (as stated in the README.md). What are your contributions to the implementation.

## Overall course goals and learning outcomes.

Comment on the alignment of the concrete course goals (taken from the course description) to the theory presented, work You have done and knowledge You have gained. (I have put some comments in [...]).

- Lexical analysis, syntax analysis, and translation into abstract syntax.

- Regular expressions and grammars, context-free languages and grammars, lexer and parser generators. [lalr-pop is a classical parser generator, it auto generated the lexer for you based on regular expressions but allows for you to define the lexer yourself for more control]

- Identifier handling and symbol table organization. Type-checking, logical inference systems. [SOS is a logical inference system]

- Intermediate representations and transformations for different languages. [If you attended, Recall lectures relating LLVM/Crane-lift, discussions on SSA (Single Static Assignment) used in LLVM/Crane-lift, and discussions/examples on high [level optimization](https://gitlab.henriktjader.com/pln/d7050e_2020/-/tree/generics_and_traits/examples)]

- Code optimization and register allocation. Machine code generation for common architectures. [Both LLVM/Crane-Lift does the "dirty work" of backend optimization/register allocation leveraging the SSA form of the LLVM-IR]

Comment on additional things that you have experienced and learned throughout the course.
