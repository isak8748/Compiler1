use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::VecDeque;
use crate::ast::Node;
use crate::ast::Opcode;


//TODO fn_env insert and calls, comparison operators on bools

#[derive(Clone)]
//#[derive(Copy)]
#[derive(Debug)]
pub enum Types{
    Boolean,
    Number,
    UnitType,
    Unknown,
    Ref(Box<Types>),
    MutRef(Box<Types>),
}


impl Types{
    fn get_type_id(&self) -> i32{
        let id = match self {
            Types::Unknown => 0,
            Types::Boolean => 1,
            Types::Number => 2,
            Types::UnitType => 3,
            Types::Ref(_a) => 4,
            Types::MutRef(_b) => 5,
        };
        id
    }
}


pub struct FnContext{
    fn_env: HashMap<String, FnInfo>,
}

#[allow(dead_code)]
pub struct Context{
    //var_env: HashMap<String, VarInfo>,
    var_env: VecDeque<HashMap<String, VarInfo>>,
    //fn_env: HashMap<String, FnInfo>,
}

impl Context{
    fn get(&self, id: &String) -> Option<VarInfo> {
        for i in &self.var_env {
            if i.contains_key(id){
                return Some(i.get(id).unwrap().clone());
            }
        };
        return None;
    }

    fn insert(&mut self, id: &String, t: &Types, mutable: &bool) {
        let mut var_info = VarInfo {t: Types::Unknown, mutable: true};
        var_info.t = t.clone();
        var_info.mutable = *mutable;
        let s = id.clone();

        if self.var_env.len() == 0 {
            let mut map = HashMap::new();
            map.insert(s, var_info);
            self.var_env.push_front(map);
        }
        else{
            self.var_env.get_mut(0).unwrap().insert(s, var_info);
        }
    }

    fn add_scope(&mut self){
        let map: HashMap<String, VarInfo>  = HashMap::new();
        self.var_env.push_front(map);
    }

    fn remove_scope(&mut self){
        self.var_env.pop_front();
    }
}
#[allow(dead_code)]
#[derive(Clone)]
#[derive(Debug)]
//#[derive(Copy)]
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
    let v = VecDeque::new();
    let A = VarInfo {t: Types::Number, mutable: true};
    let B: VarInfo = VarInfo {t: Types::Number, mutable: false};
    let D: VarInfo = VarInfo {t: Types::Boolean, mutable: true};
    let mut c: Context = Context {var_env: v};
    c.insert(&"A".to_string(), &A.t, &A.mutable);
    c.insert(&"B".to_string(), &B.t, &B.mutable);
    c.insert(&"D".to_string(), &D.t, &D.mutable);
    c
}

pub fn init_funcs() -> FnContext{
    let m = HashMap::new();
    let mut parameters = vec![Types::Unknown; 3];
    parameters[0] = Types::Number;
    parameters[1] = Types::Boolean;
    parameters[2] = Types::Number;
    let fn_info = FnInfo {ret: Types::Number, params: parameters};
    let mut funcs = FnContext{fn_env: m};
    funcs.fn_env.insert("foo".to_string(), fn_info);
    return funcs;
}

pub fn type_check_program(functions: &Vec<Box<Node>>) -> Result<Types, &'static str>{
    let map = HashMap::new();
    let mut funcs = FnContext{fn_env: map};
    for f in functions{
        let v = Vec::new();
        let mut info = FnInfo{ret: Types::Unknown, params: v};
        let name = match &**f{
            Node::FnDef(s, _, _, _) => s,
            _ => panic!("invalid program"),
        };
        let ret_type = match &**f{
            Node::FnDef(_, _, o, _) => o,
            _ => panic!("invalid program"),
        };
        if ret_type.is_none(){
            info.ret = Types::UnitType;
        }
        else{
            info.ret = match &ret_type.as_ref().unwrap() as &str{
                "i32" => Types::Number,
                "bool" => Types::Boolean,
                "()" => Types::UnitType,
                "&i32" => Types::Ref(Box::new(Types::Number)),
                "&bool" => Types::Ref(Box::new(Types::Boolean)),
                _ => panic!("invalid fn definition"),
            };
        }
        
        funcs.fn_env.insert(name.clone(), info);

    }
    for f in functions{
        let result = match &**f{
            Node::FnDef(name, p, o, i) => type_check_fn_def(name, p, o, i, &mut funcs),
            _ => panic!("invalid program"),
        };
        if result.is_err(){
            return result;
        }
    }
    return Ok(Types::UnitType);
}

pub fn type_check_fn_def(id: &String, params: &Vec<Box<Node>>, rtype: &Option<String>, 
    instr: &Vec<Box<Node>>, mut funcs: &mut FnContext) 
    -> Result<Types, &'static str> {
        let v = VecDeque::new();
        let mut context = Context{var_env: v};
        for p in params{
            let name = match &**p{
                Node::ParamDef(n, _) => n,
                _ => panic!("invalid params"),
            };
            let typ = match &**p{
                Node::ParamDef(_, t) => type_check_param_def(t).unwrap(),
                _ => panic!("invalid param"),
            };
            context.insert(name, &typ, &true);

        }
        println!("{:?}", context.var_env);
        //let mut context = init_context(); //@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@22
                                          //WE NEED TO INIT CONTEXT WITH THE ARGUMENTS
                                          //@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

        let ret_type: Types;
        if rtype.is_none() {
            ret_type = Types::UnitType;
        }
        else{
            ret_type = match &rtype.as_ref().unwrap() as &str{
                "i32" => Types::Number,
                "bool" => Types::Boolean,
                "()" => Types::UnitType,
                "&i32" => Types::Ref(Box::new(Types::Number)),
                "&bool" => Types::Ref(Box::new(Types::Boolean)),
                _ => panic!("invalid fn definition"),
            };
        }

        let mut parameter_defs = vec![Types::Unknown; params.len()];
        let mut i = 0;
        for param in params{
            parameter_defs[i] = type_check(param, &mut context, &mut funcs).unwrap();
            i += 1;
        }


        for n in instr{
            let e = type_check(n, &mut context, funcs);
            if e.is_err() {
                return e;
            }
        }

        let mut real_ret_type = Ok(Types::UnitType);
        let mut j = 0;
        //Setting the actual returnted type
        for n in instr{
            match &**n {
                Node::Return(_o) => real_ret_type = type_check(n, &mut context, funcs),
                Node::BlockValue(_v) => real_ret_type = type_check(n, &mut context, funcs),
                Node::IfElse(_a, _b, _c) => if j == instr.len()-1 {
                    real_ret_type = type_check(n, &mut context, funcs);
                },
                _ => (),
            };
            j += 1;
        }

        j = 0;
        //Checks if any returns are of different type than the last
        for n in instr{
            let e = type_check(n, &mut context, funcs);
            if e.is_err() {
                return e;
            }
            match &**n {
                Node::Return(_o) => if e.unwrap().get_type_id() != real_ret_type.clone().unwrap().get_type_id(){
                    return Err("Function tries to return different types");
                },
                Node::BlockValue(_v) => if e.unwrap().get_type_id() != real_ret_type.clone().unwrap().get_type_id(){
                    return Err("Function tries to return different types");
                },
                Node::IfElse(_a, _b, _c) => if j == instr.len()-1 {
                    if e.unwrap().get_type_id() != real_ret_type.clone().unwrap().get_type_id(){
                        return Err("Function tries to return different types");
                    }
                },
                _ => (),
            };
            j += 1;
        }

        
        if ret_type.get_type_id() == real_ret_type.unwrap().get_type_id(){
            let fn_info = FnInfo {ret: ret_type.clone(), params: parameter_defs};
            funcs.fn_env.insert(id.clone(), fn_info);
            return Ok(ret_type);
        }

        return Err("Mismatched return types");
}

pub fn type_check_call(id: &String, params: &Vec<Box<Node>>, mut context: &mut Context, mut funcs: &mut FnContext) -> Result<Types, &'static str>{
    let fn_info = funcs.fn_env.get(id);
    if fn_info.is_none(){
        return Err("Function called not in context");
    }
    let mut parameter_defs = vec![Types::Unknown; params.len()];
    let mut i = 0;
    for param in params{
        parameter_defs[i] = type_check(param, &mut context, &mut funcs).unwrap();
        i += 1;
    }
    let new_fn_info = funcs.fn_env.get(id);
    let mut i = 0;
    for _param in &parameter_defs{
        if parameter_defs[i].get_type_id() != new_fn_info.unwrap().params[i].get_type_id(){
            return Err("mismatched parameter types");
        }
        i += 1;
    }

    return Ok(new_fn_info.unwrap().ret.clone());
}

pub fn type_check_param_def(def: &String) -> Result<Types, &'static str> {
    let ret_type = match def as &str{
        ": i32" => Types::Number,
        ": bool" => Types::Boolean,
        ": &i32" => Types::Ref(Box::new(Types::Number)),
        ": &bool" => Types::Ref(Box::new(Types::Boolean)),
        _ => panic!("invalid param definition"),
    };
    return Ok(ret_type);
}


pub fn type_check_op(node1: &Node, operation: &Opcode, node2: &Node, context: &mut Context, mut funcs: &mut FnContext) -> Result<Types, &'static str> {
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

    let left = type_check(node1, context, funcs);
    let right = type_check(node2, context, funcs);

    if left.is_err() || right.is_err(){
        return Err("Error");
    }

    //If we are not comparing the type returned from the operation needs to be equal to both inputs
    if !compare {
        let x = left.clone();
        if left.clone().unwrap().get_type_id() == right.unwrap().get_type_id() && //both operands are of same type
        left.clone().unwrap().get_type_id() == return_type.get_type_id()          //as well as the returned type
        &&(return_type.get_type_id() ==1 || return_type.get_type_id() == 2){      //number or boolean
            return x;
        }
        else {
            return Err("Types not matched");
        }
    }

    //Otherwise we need to make sure both inputs are of equal type
    else{
        let number_type = match operation{
            Opcode::Less => true,
            Opcode::LessOrEq => true,
            Opcode::Greater => true,
            Opcode::GreaterorEq => true,
            _ => false,
        };
        if left.clone().unwrap().get_type_id() != 1 && left.clone().unwrap().get_type_id() != 2{
            return Err("invalid operands");
        }
        if number_type && (left.clone().unwrap().get_type_id() != 2 || right.clone().unwrap().get_type_id() != 2){
            return Err("This operation requires both operands to be numbers");
        }
        if left.clone().unwrap().get_type_id() == right.unwrap().get_type_id() {
            return Ok(Types::Boolean);
        }
        else {
            return Err("Types not matched");
        }
    }
}

pub fn type_check_let(id: &String, mutable: &bool, typedef: &Option<String>, value: &Option<Box<Node>>, context: &mut Context, mut funcs: &mut FnContext) 
-> Result<Types, &'static str>{
    //A type can be specified
    let left = match typedef {
        Some(s) => match s as &str {
            ": i32" => Types::Number,
            ": bool" => Types::Boolean,
            ": &i32" => Types::Ref(Box::new(Types::Number)),
            ": &bool" => Types::Ref(Box::new(Types::Boolean)),
            _ => panic!("unrecognized type"),
        },
        _ => Types::Unknown,
    };

    //Check the type of the value assigned
    let right = match value {
        Some(n) => type_check(n, context, funcs),
        _ => Err("no type"),
    };

    //Check if the value assigned had a valid type
    if right.is_ok(){
        //If no type was specified on the left side
        if left.get_type_id() == 0{
            context.insert(id, &right.clone().unwrap(), mutable);
            return right;
        }
        //If both sides match
        else if left.get_type_id() == right.clone().unwrap().get_type_id() {
            context.insert(id, &right.clone().unwrap(), mutable);
            return right;
        }
        else{
            return Err("unmatched types");
        }

    }

    //if no value was assigned at all
    if value.is_none(){
        context.insert(id, &left, mutable);
        return Ok(left);
    }

    //Otherwise we know the expression was invalid
    if right.is_err(){
        return Err("Invalid Expression");
    }

    panic!("This line should be unreachable");
}

pub fn type_check_assign(name: &String, value: &Node, context: &mut Context, mut funcs: &mut FnContext) 
-> Result<Types, &'static str>{
    if context.get(name).is_none(){
        return Err("tried assigning to non-existant variable");
    }
    let left = context.get(name).unwrap();
    if !left.mutable {
        return Err("Tried assigning a value to a nonmutable variable");
    }

    let right = type_check(value, context, funcs);
    if right.is_err() {
        return Err("Invalid expression");
    }

    if left.t.get_type_id() != right.clone().unwrap().get_type_id() && left.t.get_type_id() != 0{
        return Err("Mismatched types");
    }
    return right;
}

pub fn type_check_while(condition: &Node, instr: &Vec<Box<Node>>, context: &mut Context, mut funcs: &mut FnContext) 
-> Result<Types, &'static str>{
    let condition_type = type_check(condition, context, funcs);
    if condition_type.is_err(){
        return Err("invalid expression");
    }

    context.add_scope();
    for n in instr{
        let e = type_check(n, context, funcs);
        if e.is_err() {
            return e;
        }
    }
    context.remove_scope();

    if condition_type.unwrap().get_type_id() == 1 {
        return Ok(Types::UnitType);
    }
    return Err("While condition did not evaluate to a boolean");
}

pub fn type_check_if(condition: &Node, instr: &Vec<Box<Node>>, context: &mut Context, mut funcs: &mut FnContext) 
->  Result<Types, &'static str> {
    let condition_type = type_check(condition, context, funcs);
    if condition_type.is_err(){
        return Err("invalid expression");
    }
    if condition_type.unwrap().get_type_id() == 1 {
        if instr.len() == 0{
            return Ok(Types::UnitType);
        }
        //If the final instruction does not have a semicolon the type will be determined by this instruction
        let ret_type = match &*instr[instr.len()-1]{
            Node::BlockValue(expr) => type_check(&expr, context, funcs),
            _ => Ok(Types::UnitType),

        };

        context.add_scope();
        for n in instr{
            let e = type_check(n, context, funcs);
            if e.is_err() {
                return e;
            }
        }
        context.remove_scope();

        return ret_type;
    }
    return Err("If condition did not evaluate to a boolean"); 
}

pub fn type_check_if_else(condition: &Node, if_instr: &Vec<Box<Node>>, else_instr: &Vec<Box<Node>>, context: &mut Context, mut funcs: &mut FnContext)
->  Result<Types, &'static str>{
    let condition_type = type_check(condition, context, funcs);
    if condition_type.is_err(){
        return Err("invalid expression");
    }
    if condition_type.unwrap().get_type_id() == 1 {
       let mut if_type: Types = Types::UnitType;
       let mut else_type: Types = Types::UnitType;

       //Here the last instruction of each block is checked
       if if_instr.len() > 0 {
            if_type = match &*if_instr[if_instr.len()-1]{
                Node::BlockValue(expr) => type_check(&expr, context, funcs).unwrap(),
                _ => Types::UnitType,

            };

        }

        if else_instr.len() > 0{
            else_type = match &*else_instr[else_instr.len()-1]{
                Node::BlockValue(expr) => type_check(&expr, context, funcs).unwrap(),
                _ => Types::UnitType,
            };

        }

        context.add_scope();        
        for n in if_instr{
            let e = type_check(n, context, funcs);
            if e.is_err() {
                return e;
            }
        }
        context.remove_scope();

        context.add_scope();
        for n in else_instr{
            let e = type_check(n, context, funcs);
            if e.is_err() {
                return e;
            }
        }
        context.remove_scope();

        if if_type.get_type_id() == else_type.get_type_id(){
            return Ok(if_type);
        }
        
        else{
            return Err("if and else statement return differing types");
        }
       
    }
    return Err("If condition did not evaluate to a boolean");
}

pub fn type_check_return(node: &Option<Box<Node>>, context: &mut Context, mut funcs: &mut FnContext) -> Result<Types, &'static str>{
    let ret = match node {
        Some(b) => type_check(&b, context, funcs),
        None => Ok(Types::UnitType),
    };

    return ret;
}


pub fn type_check_unary_op(node: &Node, operation: &Opcode, context: &mut Context, mut funcs: &mut FnContext) -> Result<Types, &'static str> {
    let op_type = match operation {
        Opcode::UnarySub => Types::Number,
        Opcode::Not => Types::Boolean,
        Opcode::Ref => Types::Unknown,
        Opcode::DeRef => Types::Unknown,
        Opcode::MutRef => Types::Unknown,
        _=> panic!("Unrecognized unary op"),
    };
    //Unary -, expression needs to be a number
    if op_type.get_type_id() == 2 {
        let expr_type = type_check(node, context, funcs);
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
        let expr_type = type_check(node, context, funcs);
        if expr_type.is_err(){
            return Err("invalid expression");
        }
        if expr_type.unwrap().get_type_id() != 1 {
            return Err("Unary ! can only be used on booleans");
        }
        return Ok(Types::Boolean);
    }

    else {
        let create_ref = match operation {
            Opcode::Ref => true,
            Opcode::MutRef => true,
            _ => false,
        };

        let expr_type = type_check(node, context, funcs);
        if expr_type.is_err(){
            return Err("invalid expression");
        }

        //If we are dereferencing we need to make sure the expression is a reference
        if !create_ref{
            
            match expr_type.unwrap(){
                Types::MutRef(t) => return Ok(*t),
                Types::Ref(t) => return Ok(*t),
                _ => return Err("Tried dereferencing a non reference"),
            }
        }

        let mutable = match operation {
            Opcode::MutRef => true,
            Opcode::Ref => false,
            _ => panic!("unreachable"),
        };

        let identifier = match node{
            Node::ID(_s) => true,
            _ => false,
        };

        if !identifier{
            return Err("Tried creating a reference to a non ID");
        }

        if mutable {
            return Ok(Types::MutRef(Box::new(expr_type.unwrap())));
        }

        return Ok(Types::Ref(Box::new(expr_type.unwrap())));
        
    }
}

pub fn type_check_write_ref(op: &Opcode, id: &String, node: &Node, context: &mut Context, mut funcs: &mut FnContext) -> Result<Types, &'static str> {
    let b = match op{
        Opcode::DeRef => true,
        _ => false,
    };

    if !b{
        return Err("illegal operation for assignment");
    }

    let var = context.get(id);
    if var.is_none(){
        return Err("Unidentified variable name");
    }
    if var.clone().unwrap().t.get_type_id() != 5{
        return Err("Syntax error");
    }
    let t = match var.unwrap().t{
        Types::MutRef(t) => t,
        _=> panic!("unreachable")
    };

    if t.get_type_id() != type_check(node, context, funcs).unwrap().get_type_id(){
        return Err("mistmatched types");
    }
    return Ok(Types::Unknown);
}



#[allow(dead_code)]
#[allow(unused_variables)]
pub fn type_check(node: &Node, context: &mut Context, mut funcs: &mut FnContext) -> Result<Types, &'static str> {
    let ret = match node{
        Node::Number(_n) => Ok(Types::Number),
        Node::Boolean(_b) => Ok(Types::Boolean),
        Node::Op(l, o, r) => type_check_op(l, o, r, context, funcs),
        Node::ID(s) => Ok(context.get(s).unwrap().t),
        Node::Declaration(name, b, typedef, value) 
        => type_check_let(name, b, typedef, value, context, funcs),
        Node::Assign(s, n) => type_check_assign(s, n, context, funcs),
        Node::While(n, v) => type_check_while(n, v, context, funcs),
        Node::IfStmt(n, v) => type_check_if(n, v, context, funcs),
        Node::IfElse(n, v1, v2) => type_check_if_else(n, v1, v2, context, funcs),
        Node::Return(o) => type_check_return(o, context, funcs),
        Node::UnaryOp(op, exp) => type_check_unary_op(exp, op, context, funcs),
        Node::FnDef(id, paramvec, ret, instr) => 
        type_check_fn_def(id, paramvec, ret, instr, funcs),
        Node::ParamDef(_id, s) => type_check_param_def(s),
        Node::BlockValue(a) => type_check(a, context, funcs),
        Node::Call(s, v) => type_check_call(s, v, context, funcs),
        Node::WriteByRef(op, n, v) => type_check_write_ref(op, n, v, context, funcs),
        Node::Program(v) => type_check_program(v),
        _ => Err("unknown type"),
        };
    return ret;
}