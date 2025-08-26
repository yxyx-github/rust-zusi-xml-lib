use std::path::{Path, PathBuf};
use crate::xml::zusi::lib::path::zusi_path::ZusiPath;

#[derive(PartialEq, Debug, Clone)]
pub struct PrejoinedZusiPath {
    base_path: PathBuf,
    zusi_path: ZusiPath,
    full_path: PathBuf,
}

impl PrejoinedZusiPath {
    pub fn new<P: AsRef<Path> + Into<PathBuf>>(base_path: P, zusi_path: ZusiPath) -> Self {
        let full_path = zusi_path.resolve(&base_path);

        Self {
            base_path: base_path.into(),
            zusi_path,
            full_path,
        }
    }

    pub fn base_path(&self) -> &PathBuf {
        &self.base_path
    }

    pub fn zusi_path(&self) -> &PathBuf {
        &self.zusi_path.get_path()
    }

    pub fn full_path(&self) -> &PathBuf {
        &self.full_path
    }

    pub fn into_zusi_path(self) -> ZusiPath {
        self.zusi_path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let managed_zusi_path = PrejoinedZusiPath::new("a/b", ZusiPath::new("c/d.e"));

        assert_eq!(managed_zusi_path.base_path().to_str().unwrap(), "a/b");
        assert_eq!(managed_zusi_path.zusi_path().to_str().unwrap(), "c/d.e");
        assert_eq!(managed_zusi_path.full_path().to_str().unwrap(), "a/b/c/d.e");
    }
}
