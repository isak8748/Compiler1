use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::VecDeque;
use crate::ast::Node;
use crate::ast::Opcode;


#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
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

pub fn type_check_fn_def(id: &String, params: &Vec<Box<Node>>, rtype: &Option<String>, 
    instr: &Vec<Box<Node>>, context: &Context) 
    -> Result<Types, &'static str> {
        let mut ret_type = Types::Unknown;
        if rtype.is_none() {
            ret_type = Types::UnitType;
        }
        else{
            ret_type = match &rtype.as_ref().unwrap() as &str{
                "i32" => Types::Number,
                "bool" => Types::Boolean,
                "()" => Types::UnitType,
                _ => panic!("invalid fn definition"),
            };
        }

        let mut parameter_defs = vec![Types::Unknown; params.len()];
        let mut i = 0;
        for param in params{
            parameter_defs[i] = type_check(param, context).unwrap();
            i += 1;
        }

        let mut real_ret_type = Ok(Types::Unknown);
        let mut j = 0;
        for n in instr{
            match &**n {
                Node::Return(_o) => real_ret_type = type_check(n, context),
                Node::BlockValue(_v) => real_ret_type = type_check(n, context),
                Node::IfElse(_a, _b, _c) => if j == instr.len()-1 {
                    real_ret_type = type_check(n, context);
                },
                _ => (),
            };
            j += 1;
        }

        j = 0;
        for n in instr{
            let e = type_check(n, context);
            if e.is_err() {
                return e;
            }
            match &**n {
                Node::Return(_o) => if e.unwrap().get_type_id() != real_ret_type.unwrap().get_type_id(){
                    return Err("Function tries to return different types");
                },
                Node::BlockValue(_v) => if e.unwrap().get_type_id() != real_ret_type.unwrap().get_type_id(){
                    return Err("Function tries to return different types");
                },
                Node::IfElse(_a, _b, _c) => if j == instr.len()-1 {
                    if e.unwrap().get_type_id() != real_ret_type.unwrap().get_type_id(){
                        return Err("Function tries to return different types");
                    }
                },
                _ => (),
            };
            j += 1;
        }
        
        if ret_type.get_type_id() == real_ret_type.unwrap().get_type_id(){
            return Ok(ret_type);
        }

        return Err("asfasdf");
}

pub fn type_check_param_def(def: &String, context: &Context) -> Result<Types, &'static str> {
    let ret_type = match def as &str{
        ": i32" => Types::Number,
        ": bool" => Types::Boolean,
        _ => panic!("invalid param definition"),
    };
    return Ok(ret_type);
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
            ": i32" => Types::Number,
            ": bool" => Types::Boolean,
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

pub fn type_check_while(condition: &Node, instr: &Vec<Box<Node>>, context: &Context) -> Result<Types, &'static str>{
    let condition_type = type_check(condition, context);
    if condition_type.is_err(){
        return Err("invalid expression");
    }

    for n in instr{
        let e = type_check(n, context);
        if e.is_err() {
            return e;
        }
    }

    if condition_type.unwrap().get_type_id() == 1 {
        return Ok(Types::UnitType);
    }
    return Err("While condition did not evaluate to a boolean");
}

pub fn type_check_if(condition: &Node, instr: &Vec<Box<Node>>, context: &Context) ->  Result<Types, &'static str> {
    let condition_type = type_check(condition, context);
    if condition_type.is_err(){
        return Err("invalid expression");
    }
    if condition_type.unwrap().get_type_id() == 1 {
        if instr.len() == 0{
            return Ok(Types::UnitType);
        }
        //If the final instruction does not have a semicolon the type will be determined by this instruction
        let ret_type = match &*instr[instr.len()-1]{
            Node::BlockValue(expr) => type_check(&expr, context),
            _ => Ok(Types::UnitType),

        };

        for n in instr{
            let e = type_check(n, context);
            if e.is_err() {
                return e;
            }
        }

        return ret_type;
    }
    return Err("If condition did not evaluate to a boolean"); 
}

pub fn type_check_if_else(condition: &Node, if_instr: &Vec<Box<Node>>, else_instr: &Vec<Box<Node>>, context: &Context)
->  Result<Types, &'static str>{
    let condition_type = type_check(condition, context);
    if condition_type.is_err(){
        return Err("invalid expression");
    }
    if condition_type.unwrap().get_type_id() == 1 {
       let mut if_type: Types = Types::UnitType;
       let mut else_type: Types = Types::UnitType;

       //Here the last instruction of each block is checked
       if if_instr.len() > 0 {
            if_type = match &*if_instr[if_instr.len()-1]{
                Node::BlockValue(expr) => type_check(&expr, context).unwrap(),
                _ => Types::UnitType,

            };

        }

        if else_instr.len() > 0{
            else_type = match &*else_instr[else_instr.len()-1]{
                Node::BlockValue(expr) => type_check(&expr, context).unwrap(),
                _ => Types::UnitType,
            };

        }

        for n in if_instr{
            let e = type_check(n, context);
            if e.is_err() {
                return e;
            }
        }

        for n in else_instr{
            let e = type_check(n, context);
            if e.is_err() {
                return e;
            }
        }

        if if_type.get_type_id() == else_type.get_type_id(){
            return Ok(if_type);
        }
        
        else{
            return Err("if and else statement return differing types");
        }
       
    }
    return Err("If condition did not evaluate to a boolean");
}

pub fn type_check_return(node: &Option<Box<Node>>, context: &Context) -> Result<Types, &'static str>{
    let ret = match node {
        Some(b) => type_check(&b, context),
        None => Ok(Types::UnitType),
    };

    return ret;
}

pub fn type_check_unary_op(node: &Node, operation: &Opcode, context: &Context) -> Result<Types, &'static str> {
    let op_type = match operation {
        Opcode::UnarySub => Types::Number,
        Opcode::Not => Types::Boolean,
        Opcode::Ref => Types::Unknown,
        Opcode::DeRef => Types::Unknown,
        _=> panic!("Unrecognized unary op"),
    };
    //Unary -, expression needs to be a number
    if op_type.get_type_id() == 2 {
        let expr_type = type_check(node, context);
        if expr_type.is_err(){
            return Err("invalid expression");
        }
        if expr_type.unwrap().get_type_id() != 2 {
            return Err("Unary - can only be used on numbers");
        }
        return Ok(Types::Number);
    }

    //Unary !, expression needs to be a boolean
    if op_type.get_type_id() == 1 {
        let expr_type = type_check(node, context);
        if expr_type.is_err(){
            return Err("invalid expression");
        }
        if expr_type.unwrap().get_type_id() != 1 {
            return Err("Unary ! can only be used on booleans");
        }
        return Ok(Types::Boolean);
    }

    return Err("lasd");
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
        Node::While(n, v) => type_check_while(n, v, context),
        Node::IfStmt(n, v) => type_check_if(n, v, context),
        Node::IfElse(n, v1, v2) => type_check_if_else(n, v1, v2, context),
        Node::Return(o) => type_check_return(o, context),
        Node::UnaryOp(op, exp) => type_check_unary_op(exp, op, context),
        Node::FnDef(id, paramvec, ret, instr) => 
        type_check_fn_def(id, paramvec, ret, instr, context),
        Node::ParamDef(_id, s) => type_check_param_def(s, context),
        Node::BlockValue(a) => type_check(a, context),
        _ => Err("unknown type"),
        };
    return ret;
}