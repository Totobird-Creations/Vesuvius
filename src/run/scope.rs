use std::collections::HashMap;

use static_init::dynamic;

use crate::run::types::Value;


#[dynamic]
pub static mut PROGRAM_INFO : ProgramInfo = ProgramInfo::new();
pub static mut SCOPE        : Vec<Scope> = Vec::new();


#[derive(Debug)]
pub struct ProgramInfo {
    entry : Option<Vec<String>>,
}
impl ProgramInfo {
    pub fn new() -> ProgramInfo {
        return ProgramInfo {
            entry : None
        };
    }
}

#[derive(Debug)]
pub struct Scope {
    pub name    : Option<String>,
    pub symbols : HashMap<String, Symbol>
}
impl Scope {
    pub fn get_symbol(name : &String) -> &mut Symbol {
        let scope = unsafe{&mut SCOPE}; 
        let index = scope.len() - 1;
        return scope[index].symbols.get_mut(name).unwrap();
        //panic!("Unknown symbol `{}`", name);
    }
    pub fn add_symbol(name : &String, symbol : Symbol) {
        let scope = unsafe{&mut SCOPE};
        let index = scope.len() - 1;
        scope[index].symbols.insert(name.clone(), symbol);
    }

    pub fn enter_subscope(name : Option<&String>) {
        unsafe{&mut SCOPE}.push(Scope {
            name    : name.map(|x| x.clone()),
            symbols : HashMap::new()
        });
    }

    pub fn exit_subscope() {
        let scope = unsafe{&mut SCOPE};
        let index = scope.len() - 1;
        scope.remove(index);
    }

    pub fn new() -> Scope {
        return Scope {
            name    : None,
            symbols : HashMap::new()
        }
    }
}

#[derive(Debug)]
pub struct Symbol {
    pub value : Value
}
impl Symbol {
    pub fn from(value : Value) -> Symbol {
        return Symbol {
            value
        };
    }
}