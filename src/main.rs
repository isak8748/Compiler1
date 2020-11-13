use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub parser, "/parser.rs");

use parser::*;

pub mod ast;
pub mod typechecker;
pub mod interpreter;

use crate::ast::{Opcode, Node};
use crate::typechecker::{type_check_op, init_context, init_funcs, type_check, type_check_program};
use crate::interpreter::{interpret, interp_context, interp_fn_context};
use std::{env, fs};


fn main(){
    println!("{}", ExprOpParser::new().parse("+").unwrap());
    println!("{}", ExprParser::new().parse("5+6*7").unwrap());
    println!("{}", ExprParser::new().parse("22 * 44 + 66").unwrap());
    println!("{}", ExprParser::new().parse("22 + 44 * 66").unwrap());
    println!("{}", StmtParser::new().parse("x = 7").unwrap());
    println!("{:?}", StmtParser::new().parse("num = 567/14+56-14",).unwrap());
    test_parse();
    test_types();
    test_interp();
    let mut dir = env::current_dir().unwrap();
    println!("{:?}", dir);
    dir.pop();
    dir.pop();
    println!("{:?}", dir);
    dir.push("src");
    dir.push("tests");
    dir.push("sample.txt");
    let s = fs::read_to_string(dir).unwrap();
    let parse = ProgramParser::new().parse(&s);
    assert!(parse.is_ok());
    let mut c = init_context();
    let mut f = init_funcs();
    println!("type_check: {:?}", type_check(&parse.clone().unwrap(), &mut c, &mut f));
    let mut c = interp_context();
    let mut f = interp_fn_context();
    println!("interpreter: {:?}", interpret(&parse.unwrap(), &mut c, &mut f));


}


fn test_parse() {
    assert!(NumParser::new().parse("1").is_ok());
    assert!(NumParser::new().parse("1").unwrap() == 1);
    assert!(IdParser::new().parse("hello").is_ok());
    assert!(IdParser::new().parse("hello").unwrap() == "hello");
    assert!(TermParser::new().parse("(123)").is_ok());
    assert!(ExprParser::new().parse("22 * 44+66").is_ok());
    assert!(ExprParser::new().parse("((13*(51)))+561*((13)+567)/(asdf)").is_ok());
    assert!(StmtParser::new().parse("x = 5").is_ok());
    assert!(DeclarationParser::new().parse("let y = 4").is_ok());
    assert!(FunctionCallParser::new().parse("fib(56, 12, roger)").is_ok());
    assert!(FunctionCallParser::new().parse("fib(56, 12, roger, 45+18,)").is_ok());
    assert!(IfParser::new().parse("if x + 5 { fib(56,); let y = 4;}").is_ok());
    assert!(IfParser::new().parse("if x + 5 { fib(56,); y = 4;}").is_ok());
    assert!(IfElseParser::new().parse("if bool { fib(56, rususu); let y = 3;} else {let y = 456; foo(123, 132, 555);}").is_ok());
    assert!(WhileParser::new().parse("while false { print(123, rogerd); let x = 17; }").is_ok());
    assert!(WhileParser::new().parse("while true { let x = 5; if(true) {let x = 6;} else {let y = 7; if(x +5){ x = 7}}}").is_ok());
    assert!(WhileParser::new().parse("while true {}").is_ok());
    assert!(ComparisonParser::new().parse("x+123/14 <= y-15").is_ok());
    assert!(ComparisonParser::new().parse("boolname").is_ok());
    assert!(BoolExpParser::new().parse("x == 5 && y < 17 || z <= x").is_ok());
    assert!(TypeSpecParser::new().parse(": i32").is_ok());
    assert!(FunctionParser::new().parse("plus_one() -> i32 {let y = x + 1;}").is_ok());
    assert!(FunctionParser::new().parse("main() {
        let x = plus_one(5); println(y, x);}").is_ok());
    assert!(ReturnParser::new().parse("return").is_ok());
    assert!(ReturnParser::new().parse("return x % 5").is_ok());
    assert!(ReturnParser::new().parse("return parre(x, y, true)").is_ok());
    //assert!(BoolExpParser::new().parse("-x").is_ok());
    assert!(StmtParser::new().parse("a = true ").is_ok());
    assert!(BoolExpParser::new().parse("y || a").is_ok());
    assert!(IfElseParser::new().parse("if x && y {
        let a: bool = true;
        y || a
    } else {
        x && false
    }").is_ok());
    assert!(FunctionParser::new().parse("c(x: bool, y: bool) -> i32 {
        let mut b: i32 = 0;
        let mut c: i32 = 1;
        while (b < 10) {
            c = c * 2;
        };
        c 
    }").is_ok());
    assert!(BoolExpParser::new().parse("true && false").is_ok());
    println!("{:?}", BoolExpParser::new().parse("true && false").unwrap());
    println!("{:?}", BoolExpParser::new().parse("D == false && 14 < 17 || A <= B").unwrap());
    println!("{:?}", DeclarationParser::new().parse("let x: i32 = fib(13, 5)").unwrap());
    println!("{:?}", IfParser::new().parse("if x {let a = 5; let b = 3; a + 5}").unwrap());
    println!("{:?}", BoolExpParser::new().parse("!(a && true) || !isTrue(d, s, d,)").unwrap());
    println!("{:?}", BoolExpParser::new().parse("!D && D == !false").unwrap());
    println!("{:?}", FunctionParser::new().parse("fib(a: bool, d: bool) -> i32 {
        let x = 5;
        let y = &mut x;
        *y = 18;
        return x
    }").unwrap());
    println!("{:?}", ProgramParser::new().parse("fn c(x: bool, y: bool) -> i32 {
        let mut b: i32 = 0;
        let mut c: i32 = 1;
        while (b < 10) {
            c = c * 2;
        };
        c 
    }
    fn main (i: i32){
        c(true, false);
        return;
    }    
    
    "
    ));
    assert!(ProgramParser::new().parse("fn c(x: bool, y: bool) -> i32 {
        let mut b: i32 = 0;
        let mut c: i32 = 1;
        while (b < 10) {
            c = c * 2;
        };
        c 
    }
    fn main (i: i32){
        c(true, false);
        return;
    }    
    
    "
    ).is_ok());
    
  }



  fn test_types(){
    let mut c = init_context();
    let mut funcs = init_funcs();
    assert!(type_check_op(&Node::Number(14), &Opcode::Add, &Node::Number(13), &mut c, &mut funcs).is_ok());
    let n1 = Node::Op(Box::new(Node::Number(13)), Opcode::Add, Box::new(Node::Number(13)));
    let n2 = Node::Op(Box::new(Node::Number(15)), Opcode::Sub, Box::new(Node::Number(14)));
    assert!(type_check_op(&n1, &Opcode::Add, &n2, &mut c, &mut funcs).is_ok());
    assert!(type_check_op(&n1, &Opcode::Mod, &Node::ID("A".to_string()), &mut c, &mut funcs).is_ok());
    assert!(type_check_op(&n1, &Opcode::Mod, &Node::ID("D".to_string()), &mut c, &mut funcs).is_err());
    let n3 = Box::new(Node::Op(Box::new(Node::Number(13)), Opcode::Neq, Box::new(Node::Number(56))));
    let n4 = Node::Op(n3, Opcode::And, Box::new(Node::Boolean(true)));
    assert!(type_check(&n4, &mut c, &mut funcs).is_ok());
    assert!(type_check_op(&n1, &Opcode::And, &n2, &mut c, &mut funcs).is_err());
    assert!(type_check(&BoolExpParser::new().parse("D == false && 14 < 17 || A >= B").unwrap(), &mut c, &mut funcs).is_ok());
    assert!(type_check(&DeclarationParser::new().parse("let x").unwrap(), &mut c, &mut funcs).is_ok());
    assert!(type_check(&DeclarationParser::new().parse("let asd: i32").unwrap(), &mut c, &mut funcs).is_ok());
    assert!(type_check(&DeclarationParser::new().parse("let y = 17").unwrap(), &mut c, &mut funcs).is_ok());
    assert!(type_check(&DeclarationParser::new().parse("let x: bool = true").unwrap(), &mut c, &mut funcs).is_ok());
    assert!(type_check(&DeclarationParser::new().parse("let mut x: i32 = 34 + A").unwrap(), &mut c, &mut funcs).is_ok());
    assert!(type_check(&StmtParser::new().parse("A = 5").unwrap(), &mut c, &mut funcs).is_ok());
    assert!(type_check(&StmtParser::new().parse("A = true").unwrap(), &mut c, &mut funcs).is_err());
    assert!(type_check(&StmtParser::new().parse("D = false").unwrap(), &mut c, &mut funcs).is_ok());
    assert!(type_check(&StmtParser::new().parse("A = A % 123 + 45 * 3").unwrap(), &mut c, &mut funcs).is_ok());
    assert!(type_check(&WhileParser::new().parse("while true {let x = 5}").unwrap(), &mut c, &mut funcs).is_ok());
    assert!(type_check(&WhileParser::new().parse("while D && 13 <= A || 456 != B {let x = 5}").unwrap(), &mut c, &mut funcs).is_ok());
    assert!(type_check(&WhileParser::new().parse("while A % 4 + true {let x = 5}").unwrap(), &mut c, &mut funcs).is_err());
    assert!(type_check(&WhileParser::new().parse("while A % 4 * 32 {let x = 5}").unwrap(), &mut c, &mut funcs).is_err());
    println!("{:?}", type_check(&IfParser::new().parse("if true { let x = 5; 1234 * 12}").unwrap(), &mut c, &mut funcs));
    println!("{:?}", type_check(&IfParser::new().parse("if true { let x = 5; let a = 7;}").unwrap(), &mut c, &mut funcs));
    println!("{:?}", type_check(&IfElseParser::new().parse("if true{5} else {6}").unwrap(), &mut c, &mut funcs));
    assert!(type_check(&IfElseParser::new().parse("if true{5} else {6}").unwrap(), &mut c, &mut funcs).is_ok());
    assert!(type_check(&IfElseParser::new().parse("if false {13} else {true}").unwrap(), &mut c, &mut funcs).is_err());
    assert!(type_check(&IfElseParser::new().parse("if D || true {let x = 3; 14 %3} else {let x = 3; 67-1}").unwrap(), &mut c, &mut funcs).is_ok());
    assert!(type_check(&IfElseParser::new().parse("if D || true {let x = 3 + true; 14 %3} else {let x = 4; D}").unwrap(), &mut c, &mut funcs).is_err());
    assert!(type_check(&DeclarationParser::new().parse("let mut x: i32 = if D {13%10} else{A}").unwrap(), &mut c, &mut funcs).is_ok());
    assert!(type_check(&ReturnParser::new().parse("return").unwrap(), &mut c, &mut funcs).is_ok());
    assert!(type_check(&ReturnParser::new().parse("return A + 12313 * 123").unwrap(), &mut c, &mut funcs).is_ok());
    assert!(type_check(&ReturnParser::new().parse("return 3 && 14").unwrap(), &mut c, &mut funcs).is_err());
    assert!(type_check(&BoolExpParser::new().parse("-5 + 13 % (-A + -12)").unwrap(), &mut c, &mut funcs).is_ok());
    assert!(type_check(&BoolExpParser::new().parse("!D && D == !false").unwrap(), &mut c, &mut funcs).is_ok());
    assert!(type_check(&FunctionParser::new().parse("fib(a: bool, d: bool) -> i32 {
        let a = 5; if true {6123} else {1234 % 123}}").unwrap(), &mut c, &mut funcs).is_ok());
    assert!(type_check(&DeclarationParser::new().parse("let x = 3 + true").unwrap(), &mut c, &mut funcs).is_err());
    assert!(type_check(&WhileParser::new().parse("while true {let x = 5; return 12;}").unwrap(), &mut c, &mut funcs).is_ok());
    assert!(type_check(&WhileParser::new().parse("while D && 13 <= A || 456 != B {let A = true; return A || false;}").unwrap(), &mut c, &mut funcs).is_ok());
    assert!(type_check(&WhileParser::new().parse("while D && 13 <= A || 456 != B {let A = true; if A && false {return A || true}}").unwrap(), &mut c, &mut funcs).is_ok());
    assert!(type_check(&WhileParser::new().parse("while true {let A = true; let c = &A}").unwrap(), &mut c, &mut funcs).is_ok());
    assert!(type_check(&WhileParser::new().parse("while true {let A = true; let c = *A}").unwrap(), &mut c, &mut funcs).is_err());
    assert!(type_check(&WhileParser::new().parse("while true {let A = true; let c = &A; let b = *c}").unwrap(), &mut c, &mut funcs).is_ok());
    assert!(type_check(&WhileParser::new().parse("while true {let A = true; let c = &mut A; let b = *c}").unwrap(), &mut c, &mut funcs).is_ok());
    assert!(type_check(&WhileParser::new().parse("while true {let A = true; let c = &A; let b = *c}").unwrap(), &mut c, &mut funcs).is_ok());
    assert!(type_check(&WhileParser::new().parse("while true {let A: i32 = 5; let c: &i32 = &A; let b = *c}").unwrap(), &mut c, &mut funcs).is_ok());
    println!("@@@@@@@@@@@@@@@@@@@@@@@@@");
    assert!(type_check(&FunctionParser::new().parse("fib(a: bool, d: bool) -> i32 {
        let x = 1+4;
        let y = true || false;
        let z = 2345;
        let a: i32 = foo(1 + 4, true || false, z + 1);
        a
    }").unwrap(), &mut c, &mut funcs).is_ok());
    assert!(type_check(&FunctionParser::new().parse("fib(a: bool, d: bool) -> i32 {
        let x = 5;
        let y = &mut x;
        *y = 18;
        return x
    }").unwrap(), &mut c, &mut funcs).is_ok());
    assert!(type_check(&WhileParser::new().parse("while false{let a: bool = true <= false}").unwrap(), &mut c, &mut funcs).is_err());
    assert!(type_check(&WhileParser::new().parse("while false{let a: bool = 5 && 6}").unwrap(), &mut c, &mut funcs).is_err());
    assert!(type_check(&WhileParser::new().parse("while false{let a: bool = false; let a = &5}").unwrap(), &mut c, &mut funcs).is_err());
    println!("QQQQQQQQQQQQQQQQQQQQQQQQQQQ");
    println!("{:?}", type_check(&ProgramParser::new().parse("fn fib(x: i32) -> i32{
        let x = 2;
        return 4 + x;
    }
    fn main(){
        let mut y = true;
        while true{
            y = false
        };
        fib(2345);
    }
    ").unwrap(), &mut c, &mut funcs));    
    assert!(type_check(&ProgramParser::new().parse("fn factorial(x: i32) -> i32{
        let mut ret: i32 = 0;
        if x <= 1 {
            ret = 1;
        }
        else{
          ret = x * 123 + 111;
        };
        return ret;
    }
    fn main(){
        factorial(6);
        return;
    }

    ").unwrap(), &mut c, &mut funcs).is_ok());
    assert!(type_check(&ProgramParser::new().parse("fn factorial(x: i32, y: &i32) -> i32{
        let mut ret: i32 = 0;
        if x <= 1 {
            ret = 1;
        }
        else{
          ret = x * 123 + 111;
        };
        return ret;
    }
    fn main(){
        let a = 5;
        let z = &a;
        factorial(6, z);
        return;
    }

    ").unwrap(), &mut c, &mut funcs).is_ok());
    println!("passed");

  }

  fn test_interp(){
      let mut c = interp_context();
      let mut funcs = interp_fn_context();
      assert!(interpret(&BoolExpParser::new().parse("1+2").unwrap(), &mut c, &mut funcs).is_ok());
      println!("{:?}", interpret(&BoolExpParser::new().parse("1+2").unwrap(), &mut c, &mut funcs));
      assert!(interpret(&BoolExpParser::new().parse("true == false").unwrap(), &mut c, &mut funcs).is_ok());
      println!("{:?}", interpret(&BoolExpParser::new().parse("true == false").unwrap(), &mut c, &mut funcs));
      assert!(interpret(&BoolExpParser::new().parse("23 % 4 < 345 || 15-3 == 34").unwrap(), &mut c, &mut funcs).is_ok());
      println!("{:?}", interpret(&BoolExpParser::new().parse("23 % 4 < 345 || 15-3 == 34").unwrap(), &mut c, &mut funcs));
      println!("{:?}", interpret(&BoolExpParser::new().parse("24 + -6").unwrap(), &mut c, &mut funcs));
      assert!(interpret(&DeclarationParser::new().parse("let a = 0").unwrap(), &mut c, &mut funcs).is_ok());
      let _d = interpret(&IfParser::new().parse("if true {
        let a = 2;
        let b = &a;
        let c = &mut a;
        *c = 10;
        let x = &a;
        let y = &a;
        a = 6;
        let d = *y;
    }").unwrap(), &mut c, &mut funcs);
    println!(" ok? {:?}", _d);
      let _v = interpret(&IfParser::new().parse("if 4 < 5 {
        let a = 0;
        let b = &a;
        while a < 10{
            a = a + 1;
        };
    }").unwrap(), &mut c, &mut funcs);
    
      
  }