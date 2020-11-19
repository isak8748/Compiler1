use std::collections::HashMap;
use std::collections::VecDeque;
use crate::ast::Node;
use crate::ast::Opcode;

#[derive(Clone)]
pub struct FnContext{
    fn_env: HashMap<String, FnInfo>,
}

pub struct VarContext{
    var_env: VecDeque<HashMap<String, VarInfo>>,
}

#[derive(Clone)]
struct FnInfo{
    params: Vec<Box<Node>>,
    instructions: Vec<Box<Node>>,
}

#[derive(Clone, Debug)]
struct VarInfo{
    value: Value,
    borrows: VecDeque<Borrow>,
    borrow_of: Option<String>,
}

#[derive(Clone, Debug)]
pub enum Value{
    Number(i32),
    Boolean(bool),
    RefValue,
    NoValue,
}

#[derive(Clone, Debug)]
struct Borrow{
    name: String,
    mutable: bool,
}

impl VarContext{
    fn insert_borrow(&mut self, borrow_name: &String, id: &String, is_mut: bool){
        let brws = VecDeque::new();
        let mut var_info = VarInfo{value: Value::NoValue, borrows: brws, borrow_of: None};
        let brw = Borrow {name: borrow_name.clone(), mutable: is_mut};

        let borrow_var_info = VarInfo{value: Value::NoValue, borrows: VecDeque::new(), borrow_of: Some(id.clone())};
        self.var_env[0].insert(borrow_name.clone(), borrow_var_info);

        for i in &mut self.var_env {
            if i.contains_key(id){
                let temp = i.get_mut(id).unwrap();
                if is_mut {
                    temp.borrows.clear();
                }
                temp.borrows.push_front(brw);
                var_info = temp.clone();
                break;
            }
        }

        for i in &mut self.var_env{
            if i.contains_key(id){
                i.insert(id.clone(), var_info);
                return;
            }
        }
    }

    fn remove_mut_borrow(&mut self, id: &String){
        let brws = VecDeque::new();
        let mut var_info = VarInfo{value: Value::NoValue, borrows: brws, borrow_of: None};
        let mut mut_found = false;

        for i in &mut self.var_env{
            if i.contains_key(id){
                let temp = i.get_mut(id).unwrap();
                for brw in &mut temp.borrows{
                    if brw.mutable{
                        mut_found = true;
                        temp.borrows.pop_back();
                        var_info = temp.clone();
                        break;
                    }
                }
                break;
            }
        }

        if !mut_found{
            return;
        }

        for i in &mut self.var_env{
            if i.contains_key(id){
                i.insert(id.clone(), var_info);
                return;
            }
        }
    }

    fn update(&mut self, id: &String, v: &Value){
        let brws = VecDeque::new();
        let mut var_info = VarInfo{value: Value::NoValue, borrows: brws, borrow_of: None};

        for i in &mut self.var_env {
            if i.contains_key(id){
                let temp = i.get_mut(id).unwrap();
                temp.value = v.clone();
                var_info = temp.clone();
                break;
            }
        };

        for i in &mut self.var_env{
            if i.contains_key(id){
                i.insert(id.clone(), var_info);
                return;
            }
        }
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
        let mut var_info = VarInfo {value: Value::NoValue, borrows: brws, borrow_of: None};
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

pub fn interp_fn_context() -> FnContext{
    let map = HashMap::new();
    let f = FnContext {fn_env: map};
    return f;
}

pub fn interpret_op(node1: &Node, operation: &Opcode, node2: &Node, vars: &mut VarContext, funcs: &mut FnContext) -> Result<Value, &'static str>{
    let left = interpret(node1, vars, funcs);
    let right = interpret(node2, vars, funcs);
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

pub fn interpret_program(functions: &Vec<Box<Node>>) -> Result<Value, &'static str>{
    let map = HashMap::new();
    let mut funcs = FnContext{fn_env: map};
    for f in functions{
        match &**f{
            Node::FnDef(id, params, _ret, instr) => func_definition(id, params, instr, &mut funcs), //Inserting info in the FnContext
            _ => panic!("invalid program"), 
        }
    }
    let a_var_env = VecDeque::new();
    let mut vars = VarContext{var_env: a_var_env}; //Setting up an empty context
    let v = Vec::new();
    let s: String = "main".to_string();
    let x = interpret_call_wrapper(&s, &v, &mut funcs, &mut vars);
    return x;
}


pub fn func_definition(fn_name: &String, _params: &Vec<Box<Node>>, _instructions: &Vec<Box<Node>>, funcs: &mut FnContext){
    let fn_info = FnInfo {params: _params.clone(), instructions: _instructions.clone()};
    funcs.fn_env.insert(fn_name.clone(), fn_info);
}

fn get_param_name(node: &Node) -> String{
    let s = match node{
        Node::ParamDef(name, _typespec) => name,
        _ => panic!("called on params only"),
    };
    return s.clone();
}

//Change map will map mutable references in a call to updated values. These will be updated in the callers context.
fn interpret_call_wrapper(func_name: &String, args: &Vec<Box<Node>>, funcs: &mut FnContext, vars: &mut VarContext) -> Result<Value, &'static str>{
    let mut change_map: HashMap<String, Value> = HashMap::new();
    let result = interpret_call(func_name, args, funcs, vars, &mut change_map);
    return result;
}


pub fn interpret_call(func_name: &String, args: &Vec<Box<Node>>, funcs: &mut FnContext, vars: &mut VarContext, change_map: &mut HashMap<String, Value>) 
    -> Result<Value, &'static str>{
    let mut values = Vec::new();
    for arg in args{
        values.push(interpret(arg, vars, funcs).unwrap());
    }

    let fn_info = funcs.fn_env.get_mut(func_name).unwrap().clone();
    let new_var_env = VecDeque::new();
    let mut new_vars = VarContext{var_env: new_var_env};
    new_vars.add_scope();
    

    let mut visited = vec![false; args.len()];
    let mut i = 0;
    for param in &fn_info.params{
        let mut mut_ref = false;
        let mut non_mut_ref = false;
        let type_spec = match &**param{
            Node::ParamDef(_, t) => t,
            _ => panic!("params only"),
        };

        let id = match &**param{
            Node::ParamDef(n, _) => n,
            _ => panic!("params only"),
        };

        match type_spec as &str{
            ": &i32" => non_mut_ref = true,
            ": &bool" => non_mut_ref = true,
            ": &mut i32" => mut_ref = true,
            ": &mut bool" => mut_ref = true,
            _ => mut_ref = false,
        };
        if non_mut_ref || mut_ref{
            visited[i] = true;
            let o = Opcode::DeRef;
            let n = Node::UnaryOp(o, args[i].clone());
            let val = interpret(&n, vars, funcs).unwrap();


            let b_name = match *args[i].clone(){
                Node::ID(s) => s,
                _ => panic!("unimplemented"),
            };
            new_vars.insert(&b_name.clone(), &val); //This should be changes to something which cant be overwritten

            new_vars.insert_borrow(&id, &b_name, mut_ref);
        }
        i += 1;
    };
    
    new_vars.add_scope(); //Top scope of the function
    let mut i = 0;
    for _arg in args{     //In the new context the parameter name and the argument values are inserted together
        if visited[i]{
            continue;
        }
        let param_name = get_param_name(&fn_info.params[i]);
        new_vars.insert(&param_name, &values[i]);
        i += 1;
    }
    let mut ret = Ok(Value::NoValue);
    for instr in &fn_info.instructions{
        let _x = interpret(instr, &mut new_vars, funcs); 
        match &**instr{
            Node::Return(_o) => ret = interpret(instr, &mut new_vars, funcs),
            Node::BlockValue(_v) => ret = interpret(instr, &mut new_vars, funcs),
                _ => (),
        }
    }
    return ret; //should return the actual returned value of the call
}

//add suppport for all unary operations
pub fn interpret_unary_op(operation: &Opcode, node: &Node, vars: &mut VarContext, funcs: &mut FnContext) -> Result<Value, &'static str>{
    let value = interpret(node, vars, funcs);
    let number = match operation{
        Opcode::UnarySub => true,
        _ => false,
    };

    if number{
        if value.is_err(){
            return Err("error");
        }
        let val = match value.clone().unwrap(){
            Value::Number(n) => n,
            _ => panic!("error"),
        };
    
        return Ok(Value::Number(-val));  
    }

    let boolop = match operation{
        Opcode::Not => true,
        _ => false,
    };
    
    if boolop{
        if value.is_err(){
            return Err("error");
        }
        let val = match value.clone().unwrap(){
            Value::Boolean(b) => b,
            _ => panic!("error"),
        };
        return Ok(Value::Boolean(!val)); 
    }

    let deref = match operation{
        Opcode::DeRef => true,
        _ => false,
    };

    if deref{
        let s = match node{
            Node::ID(s) => s,
            _ => panic!("should be unreachable"),
        };

        println!("-----------------------------");
        let var_info = vars.get(s);
        println!("{:?}", var_info.clone());
        let id = var_info.unwrap().borrow_of;
        println!("{:?}", id);
        let value_info = vars.get(&id.clone().unwrap()).unwrap();
        println!("{:?}", value_info);
        let mut non_mut_found = false;
        for brw in value_info.borrows{                 //Check all other borrows 
            println!("{:?}", brw.name);
            println!("{:?}", *s);
            if brw.mutable && brw.name != *s{        //If another reference exists which is mutable
                panic!("Mutable reference and another reference exists");
            }
            else if !brw.mutable && !non_mut_found{                    //If a non-mutable reference is found
                non_mut_found = true;
                if brw.name == *s{                  //If it is our reference that is ok
                    return Ok(value_info.value);
                }
                continue;
            }
            else if non_mut_found && brw.name == *s{
                if brw.mutable{                      //We know mutable and non-mutable references exist
                    panic!("Mutable reference and another reference exists");
                }
                else{
                    return Ok(value_info.value);   //Multiple non-mutable references are fine
                }
                
            }
            else if brw.name == *s{
                return Ok(value_info.value);   //Multiple non-mutable references are fine
            }

        }
        panic!("Reference not found");

    }
    return Ok(Value::RefValue); //If we are creating a reference this will signal to interpret_let or interpret_assign to handle it
}

pub fn interpret_let(id: &String, value: &Option<Box<Node>>, vars: &mut VarContext, funcs: &mut FnContext) -> Result<Value, &'static str>{
    let val = match value{
        Some(n) => interpret(n, vars, funcs),
        None => Ok(Value::NoValue),
    };

    let create_ref = match val.clone().unwrap() {
        Value::RefValue => true,
        _ => false,
    };
    if create_ref{
        create_reference(id, &value.as_ref().unwrap(), vars);
        return Ok(Value::NoValue);
    }
    vars.insert(id, &val.unwrap());
    return Ok(Value::NoValue);
}

pub fn interpret_assign(id: &String, value: &Node, vars: &mut VarContext, funcs: &mut FnContext) -> Result<Value, &'static str>{
    let val = interpret(value, vars, funcs);
    vars.update(id, &val.clone().unwrap());
    let create_ref = match val.clone().unwrap() {
        Value::RefValue => true,
        _ => false,
    };
    if create_ref{
        create_reference(id, value, vars);
        return Ok(Value::NoValue);
    }
    vars.remove_mut_borrow(id);
    return Ok(Value::NoValue);
}

fn create_reference(id: &String, node: &Node, vars: &mut VarContext){
    let op = match node{
        Node::UnaryOp(o, _n) => o,
        _ => panic!("unreachable"),
    };
    let is_mut = match op{
        Opcode::MutRef => true,
        _ => false,
    };
    let identifier = match node{
        Node::UnaryOp(_o, n) => *n.clone(),
        _ => panic!("unreachable"),
    };
    let ref_name = match identifier{
        Node::ID(s) => s,
        _ => panic!("unreachable"),
    };
    println!("{:?}", id);
    println!("{:?}", ref_name);
    vars.insert_borrow(id, &ref_name, is_mut);

}

pub fn interpret_write_ref(name: &String, node: &Node, vars: &mut VarContext, funcs: &mut FnContext) -> Result<Value, &'static str>{
        let var_info = vars.get(name);
        let id = var_info.unwrap().borrow_of;
        let value_info = vars.get(&id.clone().unwrap()).unwrap();
        let top_borrow = value_info.borrows.get(0).unwrap();
        if top_borrow.name != *name{
            panic!("mutable reference is not unique");
        }
        else if !top_borrow.mutable{
            panic!("Reference is not mutable");
        }
        else{
            let value = interpret(node, vars, funcs).unwrap();
            vars.update(&id.unwrap(), &value);
        }
        return Ok(Value::NoValue);
}

pub fn interpret_while(condition: &Node, instr: &Vec<Box<Node>>, vars: &mut VarContext, funcs: &mut FnContext)-> Result<Value, &'static str>{
    loop{
        let while_condition = interpret(condition, vars, funcs);
        let condition_value = match while_condition.unwrap(){
            Value::Boolean(b) => b,
            _ => panic!("while condition did not evaluate to boolean"),
        };
        if condition_value{
            vars.add_scope();
            for n in instr{
                let _r = interpret(n, vars, funcs);
            }
            vars.remove_scope();
        }
        else{
            break;
        }
    }
    return Ok(Value::NoValue);
}

pub fn interpret_if(condition: &Node, instr: &Vec<Box<Node>>, vars: &mut VarContext, funcs: &mut FnContext)-> Result<Value, &'static str>{
    let if_condition = interpret(condition, vars, funcs);
        let condition_value = match if_condition.unwrap(){
            Value::Boolean(b) => b,
            _ => panic!("while condition did not evaluate to boolean"),
        };
        if condition_value{
            vars.add_scope();
            for n in instr{
                let _r = interpret(n, vars, funcs);
            }
            vars.remove_scope();
        }
        return Ok(Value::NoValue);
}

pub fn interpret_if_else(condition: &Node, if_instr: &Vec<Box<Node>>, else_instr: &Vec<Box<Node>>, vars: &mut VarContext, funcs: &mut FnContext)
    -> Result<Value, &'static str>{
        let if_condition = interpret(condition, vars, funcs);
        let condition_value = match if_condition.unwrap(){
            Value::Boolean(b) => b,
            _ => panic!("while condition did not evaluate to boolean"),
        };
        if condition_value{
            vars.add_scope();
            for n in if_instr{
                let _r = interpret(n, vars, funcs);
            }
            vars.remove_scope();
        }
        else{
            vars.add_scope();
            for n in else_instr{
                let _r = interpret(n, vars, funcs);
            }
            vars.remove_scope();
        }
        return Ok(Value::NoValue); 
    }

pub fn interpret_return(node: &Option<Box<Node>>, vars: &mut VarContext, funcs: &mut FnContext) -> Result<Value, &'static str>{
    let ret = match node{
        Some(v) => interpret(&v, vars, funcs),
        None => Ok(Value::NoValue),
    };
    ret
}

pub fn interpret(node: &Node, vars: &mut VarContext, funcs: &mut FnContext) -> Result<Value, &'static str>{
    let val = match node{
        Node::Number(n) => Ok(Value::Number(*n)),
        Node::Boolean(b) => Ok(Value::Boolean(*b)),
        Node::ID(s) => Ok(vars.get(s).unwrap().value),
        Node::Op(n1, o, n2) => interpret_op(n1, o, n2, vars, funcs),
        Node::UnaryOp(o, n) => interpret_unary_op(o, n, vars, funcs),
        Node::Declaration(name, _b, _typedef, value) 
        => interpret_let(name, value, vars, funcs),
        Node::Assign(s, n) => interpret_assign(s, n, vars, funcs),
        Node::While(n, v) => interpret_while(n, v, vars, funcs),
        Node::IfStmt(n, v) => interpret_if(n, v, vars, funcs),
        Node::IfElse(n, v1, v2) => interpret_if_else(n, v1, v2, vars, funcs),
        Node::BlockValue(a) => interpret(a, vars, funcs),
        Node::Return(o) => interpret_return(o, vars, funcs),
        Node::Call(s, v) => interpret_call_wrapper(s, v, funcs, vars),
        Node::WriteByRef(_o, s, v) => interpret_write_ref(s, v, vars, funcs),
        Node::Program(v) => interpret_program(v),
        _ => panic!("err"), 
    };
    return val;

}
