use crate::xml::zusi::lib::path::zusi_path::{ZusiPath, ZusiPathError};
use std::path::{Path, PathBuf};

#[derive(PartialEq, Debug, Clone)]
pub struct PrejoinedZusiPath {
    data_dir: PathBuf,
    zusi_path: ZusiPath,
    full_path: PathBuf,
}

impl PrejoinedZusiPath {
    pub fn new<P: AsRef<Path> + Into<PathBuf>>(data_dir: P, zusi_path: ZusiPath) -> Self {
        let full_path = zusi_path.resolve(&data_dir);

        Self {
            data_dir: data_dir.into(),
            zusi_path,
            full_path,
        }
    }

    pub fn data_dir(&self) -> &PathBuf {
        &self.data_dir
    }

    pub fn zusi_path(&self) -> &ZusiPath {
        &self.zusi_path
    }

    pub fn full_path(&self) -> &PathBuf {
        &self.full_path
    }

    pub fn into_zusi_path(self) -> ZusiPath {
        self.zusi_path
    }

    pub fn join_to_zusi_path<P: AsRef<Path>>(&self, path: P) -> Result<PrejoinedZusiPath, ZusiPathError> {
        Ok(PrejoinedZusiPath::new(
            &self.data_dir,
            self.zusi_path.join(path.as_ref())?
        ))
    }
}

impl AsRef<ZusiPath> for PrejoinedZusiPath {
    fn as_ref(&self) -> &ZusiPath {
        &self.zusi_path()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_relative() {
        let prejoined_zusi_path = PrejoinedZusiPath::new("a/b", ZusiPath::new("c/d.e").unwrap());

        assert_eq!(prejoined_zusi_path.data_dir().to_str().unwrap(), "a/b");
        assert_eq!(prejoined_zusi_path.zusi_path().get().to_str().unwrap(), "c/d.e");
        assert_eq!(prejoined_zusi_path.full_path().to_str().unwrap(), "a/b/c/d.e");
    }

    #[test]
    fn test_new_absolute() {
        let prejoined_zusi_path = PrejoinedZusiPath::new("/a/b", ZusiPath::new("c/d.e").unwrap());

        assert_eq!(prejoined_zusi_path.data_dir().to_str().unwrap(), "/a/b");
        assert_eq!(prejoined_zusi_path.zusi_path().get().to_str().unwrap(), "c/d.e");
        assert_eq!(prejoined_zusi_path.full_path().to_str().unwrap(), "/a/b/c/d.e");
    }

    #[test]
    fn test_join_to_zusi_path() {
        assert_eq!(
            PrejoinedZusiPath::new("a/b", ZusiPath::new("c/d.e").unwrap()).join_to_zusi_path("f/g.h").unwrap(),
            PrejoinedZusiPath { data_dir: "a/b".into(), zusi_path: "c/d.e/f/g.h".try_into().unwrap(), full_path: "a/b/c/d.e/f/g.h".into() },
        );
        assert_eq!(
            PrejoinedZusiPath::new("/a/b", ZusiPath::new("c/d.e").unwrap()).join_to_zusi_path("f/g.h").unwrap(),
            PrejoinedZusiPath { data_dir: "/a/b".into(), zusi_path: "c/d.e/f/g.h".try_into().unwrap(), full_path: "/a/b/c/d.e/f/g.h".into() },
        );
        assert_eq!(
            PrejoinedZusiPath::new("/a/b", ZusiPath::new("c/d.e").unwrap()).join_to_zusi_path("./f/g.h").unwrap(),
            PrejoinedZusiPath { data_dir: "/a/b".into(), zusi_path: "c/d.e/f/g.h".try_into().unwrap(), full_path: "/a/b/c/d.e/f/g.h".into() },
        );
        assert_eq!(
            PrejoinedZusiPath::new("/a/b", ZusiPath::new("c/d.e").unwrap()).join_to_zusi_path("/f/g.h").unwrap_err(),
            ZusiPathError::ZusiPathMustBeRelative,
        );
    }
}
