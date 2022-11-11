//! Parsing and verification execution states.


use std::{
    path::{
        PathBuf,
        absolute
    },
    collections::HashMap
};

use crate::{
    parse::{
        get_all_modules,
        node::{
            Program,
            DeclarationType
        }
    },
    notes
};


/// The `ProgramInfo` of the current execution.
static mut PROGRAM_INFO : ProgramInfo = ProgramInfo::new();



/// Global information about the program.
pub(crate) struct ProgramInfo {
    modules : Option<HashMap<PathBuf, (String, Option<Program>)>>
}

impl ProgramInfo {

    /// Get a mutable reference to the active program info.
    pub fn get<'l>() -> &'l mut Self {
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

impl ProgramInfo {

    /// Add a module path and script to the known modules list.
    pub(crate) fn add_module(&mut self, mut path : PathBuf, script : String) {
        let modules = self.modules.as_mut().unwrap();
        path = absolute(path).unwrap();
        modules.insert(path, (script, None));
    }

    /// Add a parsed program to the known modules list.
    pub(crate) fn load_module(&mut self, mut path : PathBuf, program : Program) {
        let modules = self.modules.as_mut().unwrap();
        path = absolute(path).unwrap();
        let (_, target) = modules.get_mut(&path).unwrap();
        *target = Some(program);
        let dir = path.parent().unwrap().to_path_buf();
        for decl in &modules[&path].1.as_ref().unwrap().decls {
            if let DeclarationType::Module(subpath_parts, range) = &decl.decl {
                let mut subpath = dir.clone();
                for subpath_part in subpath_parts {
                    subpath = subpath.join(subpath_part);
                }
                if (! modules.contains_key(&subpath)) {
                    get_all_modules(Some(range.clone()), subpath);
                }
            }
        }
    }

    /// Get the script of a parsed program from the known modules list.
    pub(crate) fn script_of(&self, path : &PathBuf) -> &String {
        return &self.modules.as_ref().unwrap()[&absolute(path).unwrap()].0;
    }

}



/*struct ScopeManager {}

impl ScopeManager {

    const fn new() -> Self {
        return Self {};
    }

}*/



/// A storage cell for information about a specific module, function, etc.
pub(crate) struct Scope {}

impl Scope {

    /// Reset all systems.
    pub(crate) fn reset() {
        let mut lock = notes::global::COMPILATION_NOTES.write();
        lock.clear();
        *unsafe{&mut PROGRAM_INFO} = ProgramInfo::new();
    }

}
