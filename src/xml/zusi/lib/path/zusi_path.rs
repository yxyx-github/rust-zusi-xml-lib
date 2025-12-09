use crate::xml::zusi::lib::path::prejoined_zusi_path::PrejoinedZusiPath;
use path_clean::clean;
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum ZusiPathError {
    #[error("The path does not contain the data directory.")]
    PathDoesNotContainDataDir,

    #[error("The path must be relative.")]
    PathMustBeRelative,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ZusiPath(PathBuf);

impl ZusiPath {
    pub fn new<P: AsRef<Path> + Into<PathBuf>>(path: P) -> Result<Self, ZusiPathError> {
        if path.as_ref().is_relative() {
            Ok(Self(path.into()))
        } else {
            Err(ZusiPathError::PathMustBeRelative)
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

    pub fn parent(&self) -> Option<Self> {
        self.get().parent().map(|parent| {
            Self::new(parent).unwrap() // parent of relative path is always relative
        })
    }

    pub fn with_extension<S: AsRef<OsStr>>(&self, extension: S) -> Self {
        Self::new(self.get().with_extension(extension)).unwrap() // path is still valid, just the extension has changed
    }

    pub fn join<P: AsRef<Path>>(&self, path: P) -> Result<ZusiPath, ZusiPathError> {
        self.0.join(path.as_ref()).try_into()
    }

    pub fn clean(&mut self) {
        self.0 = clean(&self.0)
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
            ZusiPathError::PathMustBeRelative,
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
    fn test_parent() {
        assert_eq!(
            ZusiPath::new("c/d.e").unwrap().parent().unwrap(),
            ZusiPath::new("c").unwrap(),
        );
        assert_eq!(
            ZusiPath::new("c/d.e").unwrap().parent().unwrap().parent().unwrap(),
            ZusiPath::new("").unwrap(),
        );
        assert_eq!(
            ZusiPath::new("").unwrap().parent(),
            None,
        );
    }

    #[test]
    fn test_with_extension() {
        assert_eq!(
            ZusiPath::new("c/d.e").unwrap().with_extension("xml"),
            ZusiPath::new("c/d.xml").unwrap(),
        );
        assert_eq!(
            ZusiPath::new("c/d.e").unwrap().with_extension(""),
            ZusiPath::new("c/d").unwrap(),
        );
        assert_eq!(
            ZusiPath::new("").unwrap().with_extension("xml"),
            ZusiPath::new("").unwrap(),
        );
        assert_eq!(
            ZusiPath::new("").unwrap().with_extension(""),
            ZusiPath::new("").unwrap(),
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
            ZusiPathError::PathMustBeRelative,
        );
    }

    #[test]
    fn test_clean() {
        let mut path = ZusiPath::new("../a/./b//../c.d").unwrap();
        path.clean();

        assert_eq!(
            path,
            ZusiPath::new("../a/c.d").unwrap(),
        );
    }
}
