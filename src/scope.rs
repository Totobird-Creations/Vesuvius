//! Parsing and verification execution states.


use std::collections::HashMap;

use relative_path::RelativePathBuf;

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



pub(crate) fn reset() {
    let mut lock = notes::global::COMPILATION_NOTES.write();
    lock.clear();
    *unsafe{&mut PROGRAM_INFO} = ProgramInfo::new();
}



/// Global information about the program.
pub(crate) struct ProgramInfo {
    modules : Option<HashMap<RelativePathBuf, (String, Option<Program>, Option<LinkedScopes>)>>
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
    pub(crate) fn add_module(&mut self, path : RelativePathBuf, script : String) {
        let modules = self.modules.as_mut().unwrap();
        modules.insert(path, (script, None, None));
    }

    /// Add a parsed program to the known modules list.
    pub(crate) fn load_module(&mut self, path : RelativePathBuf, program : Program) {
        let modules = self.modules.as_mut().unwrap();
        let (_, target, _) = modules.get_mut(&path).unwrap();
        *target = Some(program);
        let dir = path.parent().unwrap();
        for decl in &modules[&path].1.as_ref().unwrap().decls {
            if let DeclarationType::Module(subpath_parts, range) = &decl.decl {
                let mut subpath = dir.to_owned();
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
    pub(crate) fn script_of(&self, path : &RelativePathBuf) -> &String {
        return &self.modules.as_ref().unwrap()[path].0;
    }

    /// Check all of the loaded modules.
    pub(crate) fn check_modules(&mut self) {
        let modules = self.modules.as_mut().unwrap().values_mut()
            .map(|(_, program, scopes)| {
                *scopes = Some(LinkedScopes::new());
                (program.as_mut().unwrap(), scopes)
            })
            .collect::<Vec<_>>();
        modules.into_iter().for_each(|(module, scopes)| module.register_declarations(scopes.as_mut().unwrap()))
    }

}



/// A storage cell for information about a specific module, function, etc.
pub(crate) struct LinkedScopes {}

impl LinkedScopes {

    pub fn new() -> Self {
        return Self {};
    }

}
