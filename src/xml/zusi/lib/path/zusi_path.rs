use crate::xml::zusi::lib::path::prejoined_zusi_path::PrejoinedZusiPath;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidBasePath(());

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ZusiPath(PathBuf);

impl ZusiPath {
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        Self(path.into())
    }

    pub fn new_using_base<P1: AsRef<Path>, P2: AsRef<Path>>(path: P1, base_path: P2) -> Result<Self, InvalidBasePath> {
        match path.as_ref().strip_prefix(base_path) {
            Ok(path) => Ok(Self(path.into())),
            Err(_) => Err(InvalidBasePath(())),
        }
    }

    pub fn resolve<P: AsRef<Path>>(&self, base_path: P) -> PathBuf {
        base_path.as_ref().join(&self.0)
    }
    
    pub fn get(&self) -> &PathBuf {
        &self.0
    }

    pub fn take(self) -> PathBuf {
        self.0
    }

    pub fn join<P: AsRef<Path>>(&self, path: P) -> ZusiPath {
        self.0.join(path.as_ref()).into()
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

impl From<PathBuf> for ZusiPath {
    fn from(value: PathBuf) -> Self {
        Self(value)
    }
}

impl From<&str> for ZusiPath {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

impl From<String> for ZusiPath {
    fn from(value: String) -> Self {
        Self(value.into())
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
    fn test_new_using_base() {
        assert_eq!(
            ZusiPath::new_using_base("a/b/c/d.e", "a/b").unwrap(),
            ZusiPath::new("c/d.e"),
        );
        assert_eq!(
            ZusiPath::new_using_base("a/b/c/d.e", "a/g"),
            Err(InvalidBasePath(())),
        );
    }

    #[test]
    fn test_resolve() {
        assert_eq!(
            ZusiPath::new("c/d.e").resolve("a/b"),
            PathBuf::from("a/b/c/d.e"),
        );
    }

    #[test]
    fn test_join() {
        assert_eq!(
            ZusiPath::new("a/b.c").join("d/e/f.g"),
            ZusiPath::new("a/b.c/d/e/f.g"),
        );
    }
}
