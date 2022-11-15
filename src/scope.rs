//! Parsing and verification execution states.


use std::{
    collections::HashMap,
    cell::UnsafeCell,
    fmt::{
        self,
        Debug,
        Formatter
    }
};

use relative_path::RelativePathBuf;

use crate::{
    parse::{
        get_all_modules,
        node::{
            Program,
            DeclarationType
        }
    },
    notes::{
        self,
        push_error
    },
    check::types::Value
};


/// The `ProgramInfo` of the current execution.
static mut PROGRAM_INFO : ProgramInfo<'static> = ProgramInfo::new();



pub(crate) fn reset() {
    let mut lock = notes::global::COMPILATION_NOTES.write();
    lock.clear();
    *unsafe{&mut PROGRAM_INFO} = ProgramInfo::new();
}



/// Global information about the program.
pub(crate) struct ProgramInfo<'l> {
    modules : Option<HashMap<Vec<String>, (String, Option<Program>, Option<Scope<'l>>)>>
}

impl<'l> ProgramInfo<'l> {

    /// Get a mutable reference to the active program info.
    pub fn get() -> &'static mut Self {
        let mut info = unsafe {&mut PROGRAM_INFO};
        if (matches!(info.modules, None)) {
            info.modules = Some(HashMap::new());
        }
        return info;
    }

    /// Create a new instance.
    const fn new() -> Self {
        return Self {
            modules : None
        };
    }

}

impl<'l> ProgramInfo<'l> {

    /// Add a module path and script to the known modules list.
    pub(crate) fn add_module(&mut self, module : Vec<String>, script : String) {
        let modules = self.modules.as_mut().unwrap();
        modules.insert(module, (script, None, None));
    }

    /// Add a parsed program to the known modules list.
    pub(crate) fn load_module(&mut self, base : &RelativePathBuf, module : Vec<String>, program : Program) {
        let modules = self.modules.as_mut().unwrap();
        let (_, target, _) = modules.get_mut(&module).unwrap();
        *target = Some(program);
        let mut dir = module.clone();
        dir.remove(dir.len() - 1);
        for decl in &modules[&module].1.as_ref().unwrap().decls {
            if let DeclarationType::Module(subpath_parts, range) = &decl.decl {
                let mut subpath = dir.clone();
                for subpath_part in subpath_parts {
                    subpath.push(subpath_part.clone());
                }
                if (! modules.contains_key(&subpath)) {
                    get_all_modules(Some(range.clone()), base, subpath);
                }
            }
        }
    }

    /// Get the script of a parsed program from the known modules list.
    pub(crate) fn script_of(&self, path : &Vec<String>) -> &String {
        return &self.modules.as_ref().unwrap()[path].0;
    }

    /// Check all of the loaded modules.
    pub(crate) fn check_modules(&mut self) {
        let mut modules = self.modules.as_mut().unwrap().iter_mut()
            .map(|(module, (_, program, scopes))| {
                *scopes = Some(Scope::root(module[module.len() - 1].clone()));
                (module, (program.as_mut().unwrap(), scopes))
            })
            .collect::<Vec<_>>();
        modules.iter_mut().for_each(|(_, (program, scopes))| program.register_decls(scopes.as_mut().unwrap()));
        modules.iter_mut().for_each(|(_, (program, scopes))| program.expand_types(scopes.as_mut().unwrap()));
        modules.iter_mut().for_each(|(_, (program, scopes))| program.check_contents(scopes.as_mut().unwrap()));
    }

}



/// Stores information about a specific module, function, etc.
pub(crate) struct Scope<'l> {
    name    : String,
    parent  : Option<&'l Scope<'l>>,
    symbols : UnsafeCell<HashMap<String, Value>>
}

impl<'l> Scope<'l> {

    fn new<S : Into<String>>(name : S, parent : Option<&'l Scope<'l>>) -> Self {
        return Self {
            name    : name.into(),
            parent,
            symbols : UnsafeCell::new(HashMap::new())
        };
    }

    pub(crate) fn root<S : Into<String>>(name : S) -> Self {
        return Self::new(name, None);
    }

    pub(crate) fn enter<S : Into<String>>(&'l self, name : S) -> Self {
        return Self::new(name, Some(self));
    }

}

impl<'l> Scope<'l> {

    pub(crate) fn init_symbol(&self, name : String, value : Value) {
        let symbols = unsafe{&mut*self.symbols.get()};
        if matches!(self.parent, None) && let Some(symbol) = symbols.get(&name) {
            push_error!(DuplicateSymbol, Always, {
                Some(symbol.range().clone()) => {"Already defined here."},
                Some(value.range().clone())  => {"Defined again here."}
            });
        } else {
            symbols.insert(name, value);
        }
    }

}

impl<'l> Debug for Scope<'l> {

    fn fmt(&self, f : &mut Formatter) -> fmt::Result {
        let mut parts = Vec::new();
        let mut s = Some(self);
        while let Some(t) = s {
            parts.push(t.name.clone());
            s = t.parent;
        }
        parts.reverse();
        return write!(f, "{}", parts.join("::"));
    }
    
}
