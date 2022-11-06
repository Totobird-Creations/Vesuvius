use std::collections::HashMap;
use std::cell::UnsafeCell;

use static_init::dynamic;

use crate::parse::node::Range;
use crate::run::types::Value;


#[dynamic]
pub static mut PROGRAM_INFO : ProgramInfo = ProgramInfo::new();
pub static mut SCOPE        : Vec<Scope>  = Vec::new();


#[derive(Debug)]
pub struct ProgramInfo {
    pub entry : Option<(Range, Vec<String>)>,
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
    // TODO : Do something about this unsafecell.
    // It's unsafe, possibly unsound if I screw something up elsewhere.
    pub symbols : HashMap<String, UnsafeCell<Symbol>>
}
impl Scope {
    pub fn get_symbol(name : &String) -> Option<&mut Symbol> {
        let     scopes = unsafe{&mut SCOPE}; 
        let mut index  = scopes.len() - 1;
        loop {
            let scope = &scopes[index];
            if let Some(symbol) = scope.symbols.get(name) {
                return Some(unsafe{&mut *symbol.get()});
            }
            if (index == 0) {break;}
            else {index -= 1;}
        }
        return None;
    }
    pub fn add_symbol(name : &String, symbol : Symbol) {
        let scope = unsafe{&mut SCOPE};
        let index = scope.len() - 1;
        scope[index].symbols.insert(name.clone(), UnsafeCell::new(symbol));
    }
    pub fn module_with_sub(sub : &String) -> Vec<String> {
        let mut module = Vec::new();
        for scope in unsafe{&SCOPE} {
            if let Some(name) = &scope.name {
                module.push(name.clone());
            }
        }
        module.push(sub.clone());
        return module;
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