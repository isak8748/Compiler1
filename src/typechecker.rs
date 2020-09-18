use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::VecDeque;
use crate::ast::Node;
use crate::ast::Opcode;


#[derive(Clone)]
#[derive(Copy)]
pub enum Types{
    Boolean,
    Number,
    UnitType,
    Unknown,
}

impl Types{
    fn get_type_id(&self) -> i32{
        let id = match self {
            Types::Unknown => 0,
            Types::Boolean => 1,
            Types::Number => 2,
            Types::UnitType => 3,
        };
        id
    }
}

#[allow(dead_code)]
pub struct Context{
    var_env: HashMap<String, VarInfo>,
    fn_env: HashMap<String, FnInfo>,
}
#[allow(dead_code)]
pub struct VarInfo{
    t: Types,
    mutable: bool,
}

#[allow(dead_code)]
pub struct FnInfo{
    ret: Types,
    params: Vec<Types>,
}

#[allow(non_snake_case)]
pub fn init_context() -> Context{
    let v = HashMap::new();
    let f = HashMap::new();
    let A = VarInfo {t: Types::Number, mutable: true};
    let B: VarInfo = VarInfo {t: Types::Number, mutable: false};
    let D: VarInfo = VarInfo {t: Types::Boolean, mutable: true};
    let mut c: Context = Context {var_env: v, fn_env: f};
    c.var_env.insert("A".to_string(), A);
    c.var_env.insert("B".to_string(), B);
    c.var_env.insert("D".to_string(), D);
    c
}


pub fn type_check_op(node1: &Node, operation: &Opcode, node2: &Node, context: &Context) -> Result<Types, &'static str> {
    let compare = match operation {
        Opcode::Less => true,
        Opcode::LessOrEq => true,
        Opcode::Greater => true,
        Opcode::GreaterorEq => true,
        Opcode::Equals => true,
        Opcode::Neq => true,
        _ => false,
    };

    let return_type = match operation {
        Opcode::Add => Types::Number,
        Opcode::Sub => Types::Number,
        Opcode::Mul => Types::Number,
        Opcode::Div => Types::Number,
        Opcode::Mod => Types::Number,
        _ => Types::Boolean,
    };

    let left = type_check(node1, context);
    let right = type_check(node2, context);

    if left.is_err() || right.is_err(){
        return Err("Error");
    }

    //If we are not comparing the type returned from the operation needs to be equal to both inputs
    if !compare {
        let x = left.clone();
        if left.unwrap().get_type_id() == right.unwrap().get_type_id() && 
        left.unwrap().get_type_id() == return_type.get_type_id(){
            return x;
        }
        else {
            return Err("Types not matched");
        }
    }

    //Otherwise we need to make sure both inputs are of equal type
    else{
        if left.unwrap().get_type_id() == right.unwrap().get_type_id() {
            return Ok(Types::Boolean);
        }
        else {
            return Err("Types not matched");
        }
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn type_check(node: &Node, context: &Context) -> Result<Types, &'static str> {
    let ret = match node{
        Node::Number(_n) => Ok(Types::Number),
        Node::Boolean(_b) => Ok(Types::Boolean),
        Node::Op(l, o, r) => type_check_op(l, o, r, context),
        Node::ID(s) => Ok(context.var_env.get(s).unwrap().t),
        _ => Err("unknown type"),
    };
    return ret;
}