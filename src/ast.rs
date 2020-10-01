use std::fmt;

#[derive(Debug, Clone)]
pub enum Node {
    Number(usize),
    ID(String),
    //String describes the type
    Type(String),
    //+, -, *, /, %
    Op(Box<Node>, Opcode, Box<Node>),
    //unary op
    UnaryOp(Opcode, Box<Node>),
    //Function call
    Call(String, Vec<Box<Node>>),
    //Value assigned to a variable
    Assign(String, Box<Node>),
    IfStmt(Box<Node>, Vec<Box<Node>>),
    IfElse(Box<Node>, Vec<Box<Node>>, Vec<Box<Node>>),
    While(Box<Node>, Vec<Box<Node>>),
    //Function definition
    FnDef(String, Vec<Box<Node>>, Option<String>, Vec<Box<Node>>),
    //Definition of parameters in function definition
    ParamDef(String, String),
    //Variable declaration with optional type spec and value
    Declaration(String, bool, Option<String>, Option<Box<Node>>),
    //Return
    Return(Option<Box<Node>>),
    Boolean(bool),
    BlockValue(Box<Node>),
}

#[derive(Debug, Clone)]
pub enum Opcode {
    Add, 
    Sub,
    Mul,
    Div,
    Mod, 
    Equals,
    Less,
    Greater,
    LessOrEq,
    GreaterorEq,
    Neq,
    And,
    Or,
    Not,
    UnarySub,
    Ref,
    DeRef,
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Opcode::Add => write!(f, "+"),
            Opcode::Sub => write!(f, "-"),
            Opcode::Mul => write!(f, "*"),
            Opcode::Div => write!(f, "/"),
            Opcode::Mod => write!(f, "%"),
            Opcode::Less => write!(f, "<"),
            Opcode::Greater => write!(f, ">"),
            Opcode::LessOrEq => write!(f, "<="),
            Opcode::GreaterorEq => write!(f, ">="),
            Opcode::Equals => write!(f, "=="),
            Opcode::Neq => write!(f, "!="),
            _ => panic!("error"),
        }?;
        Ok(())
    }
}


impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Node::Number(i) => write!(f, "{}", i)?,
            Node::ID(s) => write!(f, "{}", s)?,
            Node::Op(a, b, c) => write!(f, "({} {} {})", format!("{}", a), format!("{}", b), format!("{}", c))?,
            Node::Assign(a, b) => write!(f, "let {} = {};", format!("{}", a), format!("{}", b))?,
            //Expr::Call(s, v, o) => write!(f, "{}({})", format!("{}", s), format!("{}", v))?,
            //Expr::Call(s, v, o) => write!(f, "ok"),
            _ => panic!("error"),
        };
        Ok(())
    }
}