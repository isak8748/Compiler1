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

pub fn type_check_let(typedef: &Option<String>, value: &Option<Box<Node>>, context: &Context) -> Result<Types, &'static str>{
    //A type can be specified
    let left = match typedef {
        Some(s) => match s as &str {
            "i32" => Types::Number,
            "bool" => Types::Boolean,
            _ => panic!("unrecognized type"),
        },
        _ => Types::Unknown,
    };

    //Check the type of the value assigned
    let right = match value {
        Some(n) => type_check(n, context),
        _ => Err("no type"),
    };

    //Check if the value assigned had a valid type
    if right.is_ok(){
        //If no type was specified on the left side
        if left.get_type_id() == 0{
            return right;
        }
        //If both sides match
        else if left.get_type_id() == right.unwrap().get_type_id() {
            return right;
        }
        else{
            return Err("unmatched types");
        }

    }

    //if no value was assigned at all
    if value.is_none(){
        return Ok(left);
    }

    //Otherwise we know the expression was invalid
    if right.is_err(){
        return Err("Invalid Expression");
    }

    panic!("This line should be unreachable");
}

pub fn type_check_assign(name: &String, value: &Node, context: &Context) -> Result<Types, &'static str>{
    let left = context.var_env.get(name).unwrap();
    if !left.mutable {
        return Err("Tried assigning a value to a nonmutable variable");
    }

    let right = type_check(value, context);
    if right.is_err() {
        return Err("Invalid expression");
    }

    if left.t.get_type_id() != right.unwrap().get_type_id(){
        return Err("Mismatched types");
    }
    return right;
}

pub fn type_check_while(condition: &Node, context: &Context) -> Result<Types, &'static str>{
    let condition_type = type_check(condition, context);
    if condition_type.is_err(){
        return Err("invalid expression");
    }
    if condition_type.unwrap().get_type_id() == 1 {
        return Ok(Types::UnitType);
    }
    return Err("While condition did not evaluate to a boolean");
}

pub fn type_check_if(condition: &Node, context: &Context) ->  Result<Types, &'static str> {
    let condition_type = type_check(condition, context);
    if condition_type.is_err(){
        return Err("invalid expression");
    }
    if condition_type.unwrap().get_type_id() == 1 {
        return Ok(Types::UnitType);
    }
    return Err("While condition did not evaluate to a boolean"); 
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn type_check(node: &Node, context: &Context) -> Result<Types, &'static str> {
    let ret = match node{
        Node::Number(_n) => Ok(Types::Number),
        Node::Boolean(_b) => Ok(Types::Boolean),
        Node::Op(l, o, r) => type_check_op(l, o, r, context),
        Node::ID(s) => Ok(context.var_env.get(s).unwrap().t),
        Node::Declaration(_name, _b, typedef, value) 
        => type_check_let(typedef, value, context),
        Node::Assign(s, n) => type_check_assign(s, n, context),
        Node::While(n, v) => type_check_while(n, context),
        Node::IfStmt(n, v) => type_check_if(n, context),
        _ => Err("unknown type"),
        };
    return ret;
}