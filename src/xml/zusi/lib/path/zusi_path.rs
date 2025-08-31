use crate::xml::zusi::lib::path::prejoined_zusi_path::PrejoinedZusiPath;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZusiPathError {
    PathDoesNotContainDataDir,
    ZusiPathMustBeRelative,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidPathOrDataDir(());

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZusiPathMustBeRelative(());

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ZusiPath(PathBuf);

impl ZusiPath {
    pub fn new<P: AsRef<Path> + Into<PathBuf>>(path: P) -> Result<Self, ZusiPathError> {
        if path.as_ref().is_relative() {
            Ok(Self(path.into()))
        } else {
            Err(ZusiPathError::ZusiPathMustBeRelative)
        }
    }

    pub fn new_using_data_dir<P1: AsRef<Path>, P2: AsRef<Path>>(path: P1, data_dir: P2) -> Result<Self, ZusiPathError> {
        match path.as_ref().strip_prefix(data_dir) {
            Ok(path) => Ok(Self(path.into())),
            Err(_) => Err(ZusiPathError::PathDoesNotContainDataDir),
        }
    }

    pub fn resolve<P: AsRef<Path>>(&self, data_dir: P) -> PathBuf {
        data_dir.as_ref().join(&self.0)
    }
    
    pub fn get(&self) -> &PathBuf {
        &self.0
    }

    pub fn take(self) -> PathBuf {
        self.0
    }

    pub fn join<P: AsRef<Path>>(&self, path: P) -> Result<ZusiPath, ZusiPathError> {
        self.0.join(path.as_ref()).try_into()
    }
}

impl From<PrejoinedZusiPath> for ZusiPath {
    fn from(value: PrejoinedZusiPath) -> Self {
        value.into_zusi_path()
    }
}

impl From<&ZusiPath> for ZusiPath {
    fn from(value: &ZusiPath) -> Self {
        value.clone()
    }
}

impl TryFrom<PathBuf> for ZusiPath {
    type Error = ZusiPathError;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        ZusiPath::new(value)
    }
}

impl TryFrom<&str> for ZusiPath {
    type Error = ZusiPathError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        ZusiPath::new(PathBuf::from(value))
    }
}

impl TryFrom<String> for ZusiPath {
    type Error = ZusiPathError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        ZusiPath::new(PathBuf::from(value))
    }
}

impl Default for ZusiPath {
    fn default() -> Self {
        Self(PathBuf::default())
    }
}

impl AsRef<ZusiPath> for ZusiPath {
    fn as_ref(&self) -> &ZusiPath {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert!(ZusiPath::new("a/b/c.d").is_ok());
        assert_eq!(
            ZusiPath::new("/a/b/c.d").unwrap_err(),
            ZusiPathError::ZusiPathMustBeRelative,
        );
    }

    #[test]
    fn test_new_using_data_dir() {
        assert_eq!(
            ZusiPath::new_using_data_dir("a/b/c/d.e", "a/b").unwrap(),
            ZusiPath::new("c/d.e").unwrap(),
        );
        assert_eq!(
            ZusiPath::new_using_data_dir("/a/b/c/d.e", "/a/b").unwrap(),
            ZusiPath::new("c/d.e").unwrap(),
        );
        assert_eq!(
            ZusiPath::new_using_data_dir("/a/b/c/d.e", "a/b").unwrap_err(),
            ZusiPathError::PathDoesNotContainDataDir,
        );
        assert_eq!(
            ZusiPath::new_using_data_dir("a/b/c/d.e", "/a/b").unwrap_err(),
            ZusiPathError::PathDoesNotContainDataDir,
        );
        assert_eq!(
            ZusiPath::new_using_data_dir("a/b/c/d.e", "a/g").unwrap_err(),
            ZusiPathError::PathDoesNotContainDataDir,
        );
    }

    #[test]
    fn test_resolve() {
        assert_eq!(
            ZusiPath::new("c/d.e").unwrap().resolve("a/b"),
            PathBuf::from("a/b/c/d.e"),
        );
        assert_eq!(
            ZusiPath::new("c/d.e").unwrap().resolve("/a/b"),
            PathBuf::from("/a/b/c/d.e"),
        );
    }

    #[test]
    fn test_join() {
        assert_eq!(
            ZusiPath::new("a/b.c").unwrap().join("d/e/f.g").unwrap(),
            ZusiPath::new("a/b.c/d/e/f.g").unwrap(),
        );
        assert_eq!(
            ZusiPath::new("a/b.c").unwrap().join("./d/e/f.g").unwrap(),
            ZusiPath::new("a/b.c/d/e/f.g").unwrap(),
        );
        assert_eq!(
            ZusiPath::new("a/b.c").unwrap().join("/d/e/f.g").unwrap_err(),
            ZusiPathError::ZusiPathMustBeRelative,
        );
    }
}
