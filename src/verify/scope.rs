use std::{
    collections::HashMap,
    ops::{
        Deref,
        DerefMut
    },
    cell::UnsafeCell
};

use crate::parse::node::Range;


static mut PROGRAM_INFO      : ProgramInfo = ProgramInfo::new();
static mut SCOPES            : Vec<Scope>  = Vec::new();
static mut SCOPES_TO_DROP    : Vec<usize>  = Vec::new();
static mut SCOPE_GUARD_COUNT : usize       = 0;


#[derive(Debug)]
pub struct ProgramInfo {
    _entry : Option<(Range, Vec<String>)>,
}

impl ProgramInfo {
    const fn new() -> ProgramInfo {
        return ProgramInfo {
            _entry : None
        };
    }
}


pub struct ScopeGuard {
    scope     : usize,
    destroyed : bool
}

impl ScopeGuard {
    pub fn new(index : usize) -> ScopeGuard {
        *unsafe{&mut SCOPE_GUARD_COUNT} += 1;
        return ScopeGuard {
            scope     : index,
            destroyed : false
        };
    }
}

impl Deref for ScopeGuard {
    type Target = Scope;
    fn deref(&self) -> &Self::Target {
        return &unsafe{&SCOPES}[self.scope];
    }
}

impl DerefMut for ScopeGuard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut unsafe{&mut SCOPES}[self.index];
    }
}

impl Drop for ScopeGuard {
    fn drop(&mut self) {
        if (! self.destroyed) {
            self.destroyed = true;
            unsafe{&mut SCOPES_TO_DROP}.push(self.index);
            if (self.index == unsafe{&SCOPES}.len() - 1) {
                let mut index = self.index;
                while (unsafe{&SCOPES_TO_DROP}.contains(&index)) {
                    unsafe{&mut SCOPES}.remove(index);
                    unsafe{&mut SCOPES_TO_DROP}.remove(unsafe{&SCOPES_TO_DROP}.iter().position(|x| x == &index).unwrap());
                    if (index > 0) {
                        index -= 1;
                    } else {
                        break;
                    }
                }
            }
            *unsafe{&mut SCOPE_GUARD_COUNT} -= 1;
        }
    }
}


pub struct Scope {
    name    : Option<String>,
    symbols : HashMap<String, (bool, Option<Symbol>)>,
    index   : usize
}

impl Scope {

    pub fn reset() {
        if (unsafe{&SCOPE_GUARD_COUNT} <= &0) {
            *unsafe{&mut PROGRAM_INFO} = ProgramInfo::new();
            unsafe{&mut SCOPES}.clear();
            {
                let mut root = Scope::new(Some("root"));
                root.destroyed = true
            }
        } else {
            panic!("Can not reset when one or more scopes are in use.")
        }
    }

    pub fn new(name : Option<&str>) -> ScopeGuard {
        let index = unsafe{&mut SCOPES}.len();
        unsafe{&mut SCOPES}.push(Scope {
            name    : name.map(|x| x.into()),
            symbols : HashMap::new(),
            index
        });
        return ScopeGuard::new(index);
    }

}

impl Scope {

    pub fn path(&self) -> String {
        let mut name = String::new();
        for i in 0..self.index {
            if let Some(scope) = &(unsafe{&SCOPES}[i]).name {
                if (! name.is_empty()) {name += "::";}
                name += scope;
            }
        }
        return name;
    }

    pub fn symbol(&mut self, name : &str) -> SymbolGuard {
        if let Some((locked, _)) = self.symbols.get_mut(name) {
            if (*locked) {
                panic!("Can not access symbol when already in use.")
            } else {
                *locked = true;
            }
        } else {
            self.symbols.insert(name.to_string(), (true, None));
        };
        return SymbolGuard {
            scope : self.index,
            name  : name.to_string()
        };
    }

}



#[derive(Debug)]
pub struct SymbolGuard {
    scope  : usize,
    name   : String
}

impl SymbolGuard {
    pub fn create(&mut self, symbol : Symbol) {
        let symbol = &mut (unsafe{&mut SCOPES}[self.scope]).symbols.get_mut(&self.name).unwrap().1;
        if let Some(symbol) = symbol {
            panic!("Can not create already defined symbol.");
        } else {
            *symbol = symbol;
        }
    }
}

impl Deref for SymbolGuard {
    type Target = Option<Symbol>;
    fn deref(&self) -> &Self::Target {
        return &(unsafe{&SCOPES}[self.scope]).symbols[&self.name].1;
    }
}

impl<'l> DerefMut for SymbolGuard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut (unsafe{&mut SCOPES}[self.scope]).symbols.get_mut(&self.name).unwrap().1;
    }
}

impl Drop for SymbolGuard {
    fn drop(&mut self) {
        (unsafe{&mut SCOPES}[self.scope]).symbols.get_mut(&self.name).unwrap().0 = false;
    }
}


pub struct Symbol {}
