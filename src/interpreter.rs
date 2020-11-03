use std::collections::HashMap;
#[allow(unused_imports)]
use std::collections::VecDeque;
use crate::ast::Node;
use crate::ast::Opcode;

pub struct FnContext{
    fn_env: HashMap<String, Node>,
}

pub struct VarContext{
    var_env: VecDeque<HashMap<String, VarInfo>>,
}

#[derive(Clone)]
struct VarInfo{
    value: Value,
    borrows: VecDeque<Borrow>,
}

#[derive(Debug)]
#[derive(Clone)]
pub enum Value{
    Number(i32),
    Boolean(bool),
    NoValue,
}

#[derive(Clone)]
struct Borrow{
    name: String,
    mutable: bool,
}

impl VarContext{
    fn update(&mut self, id: &String, v: &Value){
        let brws = VecDeque::new();
        let mut var_info = VarInfo {value: Value::NoValue, borrows: brws};
        var_info.value = v.clone();
        let s = id.clone();

        for i in &mut self.var_env {
            if i.contains_key(id){
                i.insert(s, var_info);
                return;
            }
        };
    }

    fn get(&self, id: &String) -> Option<VarInfo> {
        for i in &self.var_env {
            if i.contains_key(id){
                return Some(i.get(id).unwrap().clone());
            }
        };
        return None;
    }

    fn insert(&mut self, id: &String, v: &Value) {
        let brws = VecDeque::new();
        let mut var_info = VarInfo {value: Value::NoValue, borrows: brws};
        var_info.value = v.clone();
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

pub fn interp_context() -> VarContext{
    let v = VecDeque::new();
    let c: VarContext = VarContext {var_env: v};
    return c;
}

pub fn interpret_op(node1: &Node, operation: &Opcode, node2: &Node, vars: &mut VarContext) -> Result<Value, &'static str>{
    let left = interpret(node1, vars);
    let right = interpret(node2, vars);
    let ret;

    if left.is_err() || right.is_err(){
        return Err("error");
    }

    let number = match left.clone().unwrap(){
        Value::Number(_n) => true,
        _ => false,
    };


    if number{
        let int1 = match left.clone().unwrap(){
            Value::Number(n) => n,
            _ => panic!("error"),
        };
    
        let int2 = match right.clone().unwrap(){
            Value::Number(n) => n,
            _ => panic!("error"),
        };
        ret = match operation{
            Opcode::Add => Value::Number(int1 + int2),
            Opcode::Sub => Value::Number(int1 - int2),
            Opcode::Mul => Value::Number(int1 * int2),
            Opcode::Div => Value::Number(int1 / int2),
            Opcode::Mod => Value::Number(int1 % int2),
            Opcode::Less => Value::Boolean(int1 < int2),
            Opcode::LessOrEq => Value::Boolean(int1 <= int2),
            Opcode::Greater => Value::Boolean(int1 > int2),
            Opcode::GreaterorEq => Value::Boolean(int1 >= int2),
            Opcode::Equals => Value::Boolean(int1 == int2),
            Opcode::Neq => Value::Boolean(int1 != int2),
            _ => panic!("Unrecognized operation for i32"),
        };

    }
    else{
        let bool1 = match left.clone().unwrap(){
            Value::Boolean(b) => b,
            _ => panic!("unrecognized type"),
        };

        let bool2 = match right.clone().unwrap(){
            Value::Boolean(b) => b,
            _ => panic!("unrecognized type"),
        };

        ret = match operation{
            Opcode::And => Value::Boolean(bool1 && bool2),
            Opcode::Or => Value::Boolean(bool1 || bool2),
            Opcode::Equals => Value::Boolean(bool1 == bool2),
            Opcode::Neq => Value::Boolean(bool1 != bool2),
            _ => panic!("unrecognized type for boolean"),
        }
    }

    return Ok(ret);
}


//add suppport for all unary operations
pub fn interpret_unary_op(operation: &Opcode, node: &Node, vars: &mut VarContext) -> Result<Value, &'static str>{
    let value = interpret(node, vars);
    if value.is_err(){
        return Err("error");
    }
    let val = match value.clone().unwrap(){
        Value::Number(n) => n,
        _ => panic!("error"),
    };
    let ret = match operation{
        Opcode::UnarySub => Value::Number(-val),
        _ => panic!("error"),
    };

    return Ok(ret);
}

pub fn interpret_let(id: &String, value: &Option<Box<Node>>, vars: &mut VarContext) -> Result<Value, &'static str>{
    let val = match value{
        Some(n) => interpret(n, vars),
        None => Ok(Value::NoValue),
    };
    vars.insert(id, &val.unwrap());
    return Ok(Value::NoValue);
}

pub fn interpret_assign(name: &String, value: &Node, vars: &mut VarContext) -> Result<Value, &'static str>{
    let val = interpret(value, vars);
    vars.update(name, &val.unwrap());
    return Ok(Value::NoValue);
}

pub fn interpret_while(condition: &Node, instr: &Vec<Box<Node>>, vars: &mut VarContext)-> Result<Value, &'static str>{
    loop{
        let while_condition = interpret(condition, vars);
        let condition_value = match while_condition.unwrap(){
            Value::Boolean(b) => b,
            _ => panic!("while condition did not evaluate to boolean"),
        };
        if condition_value{
            println!("true");
            vars.add_scope();
            for n in instr{
                let _r = interpret(n, vars);
            }
            vars.remove_scope();
        }
        else{
            break;
        }
    }
    return Ok(Value::NoValue);
}

pub fn interpret_if(condition: &Node, instr: &Vec<Box<Node>>, vars: &mut VarContext)-> Result<Value, &'static str>{
    let if_condition = interpret(condition, vars);
        let condition_value = match if_condition.unwrap(){
            Value::Boolean(b) => b,
            _ => panic!("while condition did not evaluate to boolean"),
        };
        if condition_value{
            vars.add_scope();
            for n in instr{
                let _r = interpret(n, vars);
            }
            vars.remove_scope();
        }
        return Ok(Value::NoValue);
}

pub fn interpret_if_else(condition: &Node, if_instr: &Vec<Box<Node>>, else_instr: &Vec<Box<Node>>, vars: &mut VarContext)
    -> Result<Value, &'static str>{
        let if_condition = interpret(condition, vars);
        let condition_value = match if_condition.unwrap(){
            Value::Boolean(b) => b,
            _ => panic!("while condition did not evaluate to boolean"),
        };
        if condition_value{
            vars.add_scope();
            for n in if_instr{
                let _r = interpret(n, vars);
            }
            vars.remove_scope();
        }
        else{
            vars.add_scope();
            for n in else_instr{
                let _r = interpret(n, vars);
            }
            vars.remove_scope();
        }
        return Ok(Value::NoValue); 
    }

pub fn interpret(node: &Node, vars: &mut VarContext) -> Result<Value, &'static str>{
    let val = match node{
        Node::Number(n) => Ok(Value::Number(*n)),
        Node::Boolean(b) => Ok(Value::Boolean(*b)),
        Node::ID(s) => Ok(vars.get(s).unwrap().value),
        Node::Op(n1, o, n2) => interpret_op(n1, o, n2, vars),
        Node::UnaryOp(o, n) => interpret_unary_op(o, n, vars),
        Node::Declaration(name, _b, _typedef, value) 
        => interpret_let(name, value, vars),
        Node::Assign(s, n) => interpret_assign(s, n, vars),
        Node::While(n, v) => interpret_while(n, v, vars),
        Node::IfStmt(n, v) => interpret_if(n, v, vars),
        Node::IfElse(n, v1, v2) => interpret_if_else(n, v1, v2, vars),
        _ => panic!("err"), 
    };
    return val;

}
