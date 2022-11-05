pub mod types;
pub mod verify;
pub mod notes;
pub mod scope;


pub fn reset() {
    {
        let mut lock = notes::COMPILATION_NOTES.write();
        lock.clear();
    }
    {
        let mut lock = scope::PROGRAM_INFO.write();
        *lock = scope::ProgramInfo::new();
        unsafe{&mut scope::SCOPE}.push(scope::Scope::new());
    }
    notes::push_warn!(UnstableReleaseUsed, Always, "You are using an unstable version of {}", env!("CARGO_PKG_NAME"));
}
