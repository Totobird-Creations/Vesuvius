pub mod types;
pub mod scope;
pub mod verify;

use crate::{
    notes,
    verify::scope::{
        ProgramInfo,
        Scope
    }
};


pub fn reset() {
    {
        let mut lock = notes::COMPILATION_NOTES.write();
        lock.clear();
    }
    {
        let mut lock = scope::PROGRAM_INFO.write();
        *lock = ProgramInfo::new();
        unsafe{&mut scope::SCOPE}.push(Scope::new());
    }
    notes::push_warn!(UnstableVersion, Always);
}
