use std::collections::HashMap;

use static_init::dynamic;

use crate::parse::node::Range;


#[dynamic]
pub static mut PROGRAM_INFO   : ProgramInfo = ProgramInfo::new();
    static mut SCOPES         : Vec<Scope>  = Vec::new();
    static mut SCOPES_TO_DROP : Vec<usize>  = Vec::new();


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


pub struct ScopeGuard {
    index : usize
}
impl ScopeGuard {

    pub fn get(&self) -> &mut Scope {
        return &mut unsafe{&mut SCOPES}[self.index];
    }

}
impl Drop for ScopeGuard {

    fn drop(&mut self) {
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
    }

}


#[derive(Debug)]
pub struct Scope {
    pub name : Option<String>,
    pub symbols : HashMap<String, ()>
}
impl Scope {

    pub fn root<S : Into<String>>(name : S) {
        if (unsafe{&SCOPES}.len() > 0) {
            panic!("Can not create root scope because one is already defined.");
        } else {
            unsafe{&mut SCOPES}.push(Scope {
                name    : Some(name.into()),
                symbols : HashMap::new()
            });
        }
    }

    pub fn new<S : Into<String>>(name : Option<S>) -> ScopeGuard {
        unsafe{&mut SCOPES}.push(Scope {
            name    : name.map(|x| x.into()),
            symbols : HashMap::new()
        });
        let index = unsafe{&mut SCOPES}.len() - 1;
        return ScopeGuard {
            index
        };
    }

    pub fn path() -> String {
        let mut name = String::new();
        unsafe{&SCOPES}.iter().for_each(|scope| {
            if let Some(scope_name) = &scope.name {
                if (name.len() > 0) {name += "::"};
                name += scope_name;
            }
        });
        return name;
    }

}