use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub parser, "/parser.rs");

use parser::*;

pub mod ast;
pub mod typechecker;

use crate::ast::{Opcode, Node};
use crate::typechecker::{type_check_op, init_context, type_check};

fn main() {
    println!("{}", ExprOpParser::new().parse("+").unwrap());
    println!("{}", ExprParser::new().parse("5+6*7").unwrap());
    println!("{}", ExprParser::new().parse("22 * 44 + 66").unwrap());
    println!("{}", ExprParser::new().parse("22 + 44 * 66").unwrap());
    println!("{}", StmtParser::new().parse("let x = 7").unwrap());
    println!("{:?}", StmtParser::new().parse("let num = 567/14+56-14",).unwrap());
    test_parse();
    test_types();

    
}

fn test_parse() {
    assert!(NumParser::new().parse("1").is_ok());
    assert!(NumParser::new().parse("1").unwrap() == 1);
    assert!(IdParser::new().parse("hello").is_ok());
    assert!(IdParser::new().parse("hello").unwrap() == "hello");
    assert!(TermParser::new().parse("(123)").is_ok());
    assert!(ExprParser::new().parse("22 * 44+66").is_ok());
    assert!(ExprParser::new().parse("((13*(51)))+561*((13)+567)/(asdf)").is_ok());
    assert!(StmtParser::new().parse("let x = 5").is_ok());
    assert!(FunctionCallParser::new().parse("fib(56, 12, roger)").is_ok());
    assert!(FunctionCallParser::new().parse("fib(56, 12, roger, 45+18,)").is_ok());
    assert!(IfParser::new().parse("if x + 5 { fib(56,); let y = 4}").is_ok());
    assert!(IfParser::new().parse("if x + 5 { fib(56,); y = 4;}").is_ok());
    assert!(IfElseParser::new().parse("if bool { fib(56, rususu); let y = 3;} else {let y = 456; foo(123, 132, 555);}").is_ok());
    assert!(WhileParser::new().parse("while false { print(123, rogerd); let x = 17; }").is_ok());
    assert!(WhileParser::new().parse("while true { let x = 5; if(true) {let x = 6;} else {let y = 7; if(x +5){ x = 7}}}").is_ok());
    assert!(ComparisonParser::new().parse("x+123/14 <= y-15").is_ok());
    assert!(ComparisonParser::new().parse("boolname").is_ok());
    assert!(BoolExpParser::new().parse("x == 5 && y < 17 || z <= x").is_ok());
    assert!(TypeSpecParser::new().parse(": i32").is_ok());
    assert!(FunctionParser::new().parse("fn plus_one() -> i32 {let y = x + 1;}").is_ok());
    assert!(FunctionParser::new().parse("fn main() {
        let x = plus_one(5); println(y, x);}").is_ok());
    assert!(ReturnParser::new().parse("return").is_ok());
    assert!(ReturnParser::new().parse("return x % 5").is_ok());
    assert!(ReturnParser::new().parse("return parre(x, y, true)").is_ok());
    //assert!(BoolExpParser::new().parse("-x").is_ok());
    assert!(StmtParser::new().parse("let a: i32 = true ").is_ok());
    assert!(BoolExpParser::new().parse("y || a").is_ok());
    assert!(IfElseParser::new().parse("if x && y {
        let a: bool = true;
        y || a
    } else {
        x && false
    }").is_ok());
    assert!(FunctionParser::new().parse("fn c(x: bool, y: bool) -> i32 {
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
    
  }

  fn bool_op() -> bool{
      let a = true;
      let b = false;
      if a == b {
          return true;
      }
      else{
          return false;
      }
  }

  fn test_types(){
    let c = init_context();
    assert!(type_check_op(&Node::Number(14), &Opcode::Add, &Node::Number(13), &c).is_ok());
    let n1 = Node::Op(Box::new(Node::Number(13)), Opcode::Add, Box::new(Node::Number(13)));
    let n2 = Node::Op(Box::new(Node::Number(15)), Opcode::Sub, Box::new(Node::Number(14)));
    assert!(type_check_op(&n1, &Opcode::Add, &n2, &c).is_ok());
    assert!(type_check_op(&n1, &Opcode::Mod, &Node::ID("A".to_string()), &c).is_ok());
    assert!(type_check_op(&n1, &Opcode::Mod, &Node::ID("D".to_string()), &c).is_err());
    let n3 = Box::new(Node::Op(Box::new(Node::Number(13)), Opcode::Neq, Box::new(Node::Number(56))));
    let n4 = Node::Op(n3, Opcode::And, Box::new(Node::Boolean("true".to_string())));
    assert!(type_check(&n4, &c).is_ok());
    assert!(type_check_op(&n1, &Opcode::And, &n2, &c).is_err());
    assert!(type_check(&BoolExpParser::new().parse("D == false && 14 < 17 || A >= B").unwrap(), &c).is_ok());
    assert_eq!(bool_op(), false);
  }