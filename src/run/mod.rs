pub mod types;
pub mod verify;
pub mod notes;
pub mod scope;
use scope::{
    ProgramInfo,
    Scope
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
    notes::push_warn!(UnstableRelease, Always);
}
