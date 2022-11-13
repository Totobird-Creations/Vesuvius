use std::path::{
    PathBuf,
    absolute
};

use relative_path::RelativePathBuf;


pub trait AbsolutePathBuf {
    fn absolute_from<S : Into<PathBuf> + std::fmt::Debug>(path : S) -> RelativePathBuf {
        return RelativePathBuf::from(&*absolute(path.into()).unwrap().to_string_lossy());
    }
    fn absolute(&self) -> RelativePathBuf {
        return RelativePathBuf::from(&*absolute(self.into_path()).unwrap().to_string_lossy());
    }
    fn into_path(&self) -> PathBuf;
}
impl AbsolutePathBuf for RelativePathBuf {
    fn into_path(&self) -> PathBuf {
        return PathBuf::from(self.as_str());
    }
}
