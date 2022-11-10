use std::{
    collections::HashMap,
    ops::{
        Deref,
        DerefMut
    }
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
    index : usize
}

impl ScopeGuard {
    pub fn new(index : usize) -> ScopeGuard {
        *unsafe{&mut SCOPE_GUARD_COUNT} += 1;
        return ScopeGuard {
            index
        };
    }
}

impl Deref for ScopeGuard {
    type Target = Scope;
    fn deref(&self) -> &Self::Target {
        return &unsafe{&SCOPES}[self.index];
    }
}

impl DerefMut for ScopeGuard {
    fn deref_mut(&mut self) -> &mut Self::Target {
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
        *unsafe{&mut SCOPE_GUARD_COUNT} -= 1;
    }
}


#[derive(Debug)]
pub struct Scope {
    name     : Option<String>,
    _symbols : HashMap<String, ()>,
    index    : usize
}

impl Scope {

    pub fn reset() {
        if (unsafe{&SCOPE_GUARD_COUNT} <= &0) {
            *unsafe{&mut PROGRAM_INFO} = ProgramInfo::new();
            unsafe{&mut SCOPES}.clear();
        } else {
            panic!("Can not reset when one or more scope is borrowed.")
        }
    }

    pub fn new(name : Option<&str>) -> ScopeGuard {
        let index = unsafe{&mut SCOPES}.len();
        unsafe{&mut SCOPES}.push(Scope {
            name     : name.map(|x| x.into()),
            _symbols : HashMap::new(),
            index
        });
        return ScopeGuard::new(index);
    }

}

impl Scope {

    pub fn path(&self) -> String {
        let mut name = String::new();
        for i in 0..self.index {
            if let Some(scope) = &unsafe{&SCOPES}[i].name {
                if (! name.is_empty()) {name += "::";}
                name += scope;
            }
        }
        return name;
    }

}
