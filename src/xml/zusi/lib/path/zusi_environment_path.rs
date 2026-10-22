pub mod from_to_helper_traits;

use crate::xml::zusi::lib::path::prejoined_zusi_path::PrejoinedZusiPath;
use crate::xml::zusi::lib::path::zusi_path::{ZusiPath, ZusiPathError};
use path_clean::clean;
use serde_helpers::xml::{FromXML, ReadXMLFileError, ToXML, WriteXMLFileError};
use std::ffi::OsStr;
use std::fmt::{Display, Formatter};
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZusiEnvironmentPath {
    central_data_dir: PathBuf,
    user_data_dir: PathBuf,
    zusi_path: ZusiPath,
}

impl ZusiEnvironmentPath {
    pub fn new(central_data_dir: impl Into<PathBuf>, user_data_dir: impl Into<PathBuf>) -> Self {
        Self {
            central_data_dir: central_data_dir.into(),
            user_data_dir: user_data_dir.into(),
            zusi_path: Default::default(),
        }
    }

    pub fn new_with_path(central_data_dir: impl Into<PathBuf>, user_data_dir: impl Into<PathBuf>, path: impl AsRef<Path> + Into<PathBuf>) -> Result<Self, ZusiPathError> {
        Ok(Self {
            central_data_dir: central_data_dir.into(),
            user_data_dir: user_data_dir.into(),
            zusi_path: ZusiPath::new(path)?,
        })
    }

    pub fn central_data_dir(&self) -> &Path {
        &self.central_data_dir
    }

    pub fn user_data_dir(&self) -> &Path {
        &self.user_data_dir
    }

    pub fn zusi_path(&self) -> &ZusiPath {
        &self.zusi_path
    }

    fn central_data_path(&self) -> PrejoinedZusiPath {
        PrejoinedZusiPath::new(self.central_data_dir(), self.zusi_path().clone())
    }

    fn user_data_path(&self) -> PrejoinedZusiPath {
        PrejoinedZusiPath::new(self.user_data_dir(), self.zusi_path().clone())
    }

    pub fn with_path(&self, path: impl AsRef<Path> + Into<PathBuf>) -> Result<Self, ZusiPathError> {
        Ok(
            Self {
                central_data_dir: self.central_data_dir.clone(),
                user_data_dir: self.user_data_dir.clone(),
                zusi_path: ZusiPath::new(path)?,
            }
        )
    }

    pub fn with_zusi_path(&self, zusi_path: impl Into<ZusiPath>) -> Self {
        Self {
            central_data_dir: self.central_data_dir.clone(),
            user_data_dir: self.user_data_dir.clone(),
            zusi_path: zusi_path.into(),
        }
    }

    pub fn join(&self, path: impl AsRef<Path> + Into<PathBuf>) -> Result<Self, ZusiPathError> {
        Ok(
            Self {
                central_data_dir: self.central_data_dir.clone(),
                user_data_dir: self.user_data_dir.clone(),
                zusi_path: self.zusi_path().join(path)?,
            }
        )
    }

    pub fn parent(&self) -> Option<Self> {
        self.zusi_path().parent().map(|parent_zusi_path| {
            Self {
                central_data_dir: self.central_data_dir.clone(),
                user_data_dir: self.user_data_dir.clone(),
                zusi_path: parent_zusi_path,
            }
        })
    }

    pub fn with_extension(&self, extension: impl AsRef<OsStr>) -> Self {
        Self {
            central_data_dir: self.central_data_dir.clone(),
            user_data_dir: self.user_data_dir.clone(),
            zusi_path: self.zusi_path().with_extension(extension),
        }
    }

    pub fn clean(&mut self) {
        self.central_data_dir = clean(self.central_data_dir());
        self.user_data_dir = clean(self.user_data_dir());
        self.zusi_path.clean();
    }

    fn read_path(&self) -> io::Result<PrejoinedZusiPath> {
        let user_data_path = self.user_data_path();
        if user_data_path.full_path().try_exists()? {
            Ok(user_data_path)
        } else {
            Ok(self.central_data_path())
        }
    }

    fn write_path(&self) -> PrejoinedZusiPath {
        self.user_data_path()
    }

    pub fn read_xml_file<T: FromXML>(&self) -> Result<T, ReadXMLFileError> {
        T::from_xml_file_by_path(self.read_path()?.full_path())
    }

    pub fn write_xml_file<T: ToXML>(&self, object: T, declaration_header: bool) -> Result<(), WriteXMLFileError> {
        object.to_xml_file_by_path(self.write_path().full_path(), declaration_header)
    }
}

impl Display for ZusiEnvironmentPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}, {}", self.central_data_path().full_path().display(), self.user_data_path().full_path().display())
    }
}

impl From<&ZusiEnvironmentPath> for ZusiEnvironmentPath {
    fn from(value: &ZusiEnvironmentPath) -> Self {
        value.clone()
    }
}

impl AsRef<ZusiEnvironmentPath> for ZusiEnvironmentPath {
    fn as_ref(&self) -> &ZusiEnvironmentPath {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xml::zusi::lib::datei::Datei;
    use crate::xml::zusi::zug::Zug;
    use crate::xml::zusi::zug::fahrzeug_varianten::FahrzeugVarianten;
    use crate::xml::zusi::{TypedZusi, Zusi};
    use serde_helpers::xml::test_utils::{cleanup_xml, read_xml_file};
    use tempfile::tempdir;
    use test_helpers::tmp_dir_helper::TmpDirHelper;
    use test_helpers::tmp_dir_helper::tmp_dir_input::list_input::ListInputValue;

    #[test]
    fn test_new() {
        let tmp_dir = tempdir().unwrap();

        let central_dir = tmp_dir.path().join("central_dir");
        let user_dir = tmp_dir.path().join("user_dir");
        let env = ZusiEnvironmentPath::new(&central_dir, &user_dir);

        assert_eq!(
            *env.central_data_dir(),
            central_dir,
        );
        assert_eq!(
            *env.user_data_dir(),
            user_dir,
        );
        assert_eq!(
            *env.zusi_path(),
            ZusiPath::new("").unwrap(),
        );
    }

    #[test]
    fn test_new_with_path() {
        let tmp_dir = tempdir().unwrap();

        let central_dir = tmp_dir.path().join("central_dir");
        let user_dir = tmp_dir.path().join("user_dir");
        let env = ZusiEnvironmentPath::new_with_path(&central_dir, &user_dir, "path/to/file").unwrap();

        assert_eq!(
            *env.central_data_dir(),
            central_dir,
        );
        assert_eq!(
            *env.user_data_dir(),
            user_dir,
        );
        assert_eq!(
            *env.zusi_path(),
            ZusiPath::new("path/to/file").unwrap(),
        );
    }

    #[test]
    fn test_central_and_user_data_path() {
        let tmp_dir = tempdir().unwrap();

        let central_dir = tmp_dir.path().join("central_dir");
        let user_dir = tmp_dir.path().join("user_dir");
        let env = ZusiEnvironmentPath::new(&central_dir, &user_dir).with_path("path/to/file").unwrap();

        assert_eq!(
            *env.central_data_dir(),
            central_dir,
        );
        assert_eq!(
            *env.user_data_dir(),
            user_dir,
        );
        assert_eq!(
            *env.zusi_path(),
            ZusiPath::new("path/to/file").unwrap(),
        );
        assert_eq!(
            env.central_data_path(),
            PrejoinedZusiPath::new(central_dir, ZusiPath::new("path/to/file").unwrap()),
        );
        assert_eq!(
            env.user_data_path(),
            PrejoinedZusiPath::new(user_dir, ZusiPath::new("path/to/file").unwrap()),
        );
    }

    #[test]
    fn test_with_path() {
        let tmp_dir = tempdir().unwrap();

        let central_dir = tmp_dir.path().join("central_dir");
        let user_dir = tmp_dir.path().join("user_dir");
        let env = ZusiEnvironmentPath::new(&central_dir, &user_dir).with_path("path/to/file").unwrap();

        assert_eq!(
            *env.central_data_dir(),
            central_dir,
        );
        assert_eq!(
            *env.user_data_dir(),
            user_dir,
        );
        assert_eq!(
            *env.zusi_path(),
            ZusiPath::new("path/to/file").unwrap(),
        );

        let env = env.with_path("path/to/other/file").unwrap();

        assert_eq!(
            *env.zusi_path(),
            ZusiPath::new("path/to/other/file").unwrap(),
        );
    }

    #[test]
    fn test_with_zusi_path() {
        let tmp_dir = tempdir().unwrap();

        let central_dir = tmp_dir.path().join("central_dir");
        let user_dir = tmp_dir.path().join("user_dir");
        let env = ZusiEnvironmentPath::new(&central_dir, &user_dir).with_zusi_path(ZusiPath::new("path/to/file").unwrap());

        assert_eq!(
            *env.central_data_dir(),
            central_dir,
        );
        assert_eq!(
            *env.user_data_dir(),
            user_dir,
        );
        assert_eq!(
            *env.zusi_path(),
            ZusiPath::new("path/to/file").unwrap(),
        );

        let env = env.with_path("path/to/other/file").unwrap();

        assert_eq!(
            *env.zusi_path(),
            ZusiPath::new("path/to/other/file").unwrap(),
        );
    }

    #[test]
    fn test_join() {
        let tmp_dir = tempdir().unwrap();

        let central_dir = tmp_dir.path().join("central_dir");
        let user_dir = tmp_dir.path().join("user_dir");
        let env = ZusiEnvironmentPath::new(&central_dir, &user_dir).join("path/to/file").unwrap();

        assert_eq!(
            *env.central_data_dir(),
            central_dir,
        );
        assert_eq!(
            *env.user_data_dir(),
            user_dir,
        );
        assert_eq!(
            *env.zusi_path(),
            ZusiPath::new("path/to/file").unwrap(),
        );

        let env = env.join("and/somewhere/else").unwrap();

        assert_eq!(
            *env.zusi_path(),
            ZusiPath::new("path/to/file/and/somewhere/else").unwrap(),
        );
    }

    #[test]
    fn test_parent() {
        let tmp_dir = tempdir().unwrap();

        let central_dir = tmp_dir.path().join("central_dir");
        let user_dir = tmp_dir.path().join("user_dir");
        let env = ZusiEnvironmentPath::new(&central_dir, &user_dir).with_path("path/to/file").unwrap().parent().unwrap();

        assert_eq!(
            *env.central_data_dir(),
            central_dir,
        );
        assert_eq!(
            *env.user_data_dir(),
            user_dir,
        );
        assert_eq!(
            *env.zusi_path(),
            ZusiPath::new("path/to").unwrap(),
        );

        assert_eq!(env.parent().unwrap().parent().unwrap().parent(), None);
    }

    #[test]
    fn test_with_extension() {
        let tmp_dir = tempdir().unwrap();

        let central_dir = tmp_dir.path().join("central_dir");
        let user_dir = tmp_dir.path().join("user_dir");
        let env = ZusiEnvironmentPath::new(&central_dir, &user_dir).with_path("path/to/file.yml").unwrap().with_extension("xml");

        assert_eq!(
            *env.central_data_dir(),
            central_dir,
        );
        assert_eq!(
            *env.user_data_dir(),
            user_dir,
        );
        assert_eq!(
            *env.zusi_path(),
            ZusiPath::new("path/to/file.xml").unwrap(),
        );
    }

    #[test]
    fn test_clean() {
        let tmp_dir = tempdir().unwrap();

        let central_dir = tmp_dir.path().join("a/./b//../c");
        let user_dir = tmp_dir.path().join("x/..//y/./z");
        let mut env = ZusiEnvironmentPath::new(&central_dir, &user_dir).with_path("../d/./e//../f.g").unwrap();
        env.clean();

        assert_eq!(
            *env.central_data_dir(),
            tmp_dir.path().join("a/c"),
        );
        assert_eq!(
            *env.user_data_dir(),
            tmp_dir.path().join("y/z"),
        );
        assert_eq!(
            *env.zusi_path(),
            ZusiPath::new("../d/f.g").unwrap(),
        );
    }

    #[test]
    fn test_read_path_that_exists_in_user_dir() {
        let tmp_dir_helper = TmpDirHelper::<ListInputValue>::from(vec![
            ("central_dir/test", r#"
                test
            "#),
            ("user_dir/test", r#"
                test
            "#),
        ]).unwrap();

        let central_dir = tmp_dir_helper.path().join("central_dir");
        let user_dir = tmp_dir_helper.path().join("user_dir");
        let env = ZusiEnvironmentPath::new(&central_dir, &user_dir).with_path("test").unwrap();

        assert_eq!(
            env.read_path().unwrap(),
            PrejoinedZusiPath::new(user_dir, ZusiPath::new("test").unwrap()),
        );
    }

    #[test]
    fn test_read_path_that_does_not_exist_in_user_dir() {
        let tmp_dir_helper = TmpDirHelper::<ListInputValue>::from(vec![
            ("central_dir/test", r#"
                test
            "#),
            ("user_dir/testxyz", r#"
                testxyz
            "#),
        ]).unwrap();

        let central_dir = tmp_dir_helper.path().join("central_dir");
        let user_dir = tmp_dir_helper.path().join("user_dir");
        let env = ZusiEnvironmentPath::new(&central_dir, &user_dir).with_path("test").unwrap();

        assert_eq!(
            env.read_path().unwrap(),
            PrejoinedZusiPath::new(central_dir, ZusiPath::new("test").unwrap()),
        );
    }

    #[test]
    fn test_write_path() {
        let tmp_dir_helper = TmpDirHelper::<ListInputValue>::from(vec![
            ("central_dir/test", r#"
                test
            "#),
            ("user_dir/testxyz", r#"
                testxyz
            "#),
        ]).unwrap();

        let central_dir = tmp_dir_helper.path().join("central_dir");
        let user_dir = tmp_dir_helper.path().join("user_dir");
        let env = ZusiEnvironmentPath::new(&central_dir, &user_dir).with_path("test").unwrap();

        assert_eq!(
            env.write_path(),
            PrejoinedZusiPath::new(user_dir, ZusiPath::new("test").unwrap()),
        );
    }

    #[test]
    fn test_read_xml_file_that_exists_in_user_dir() {
        let tmp_dir_helper = TmpDirHelper::<ListInputValue>::from(vec![
            ("central_dir/test.trn", r#"
                <?xml version="1.0" encoding="UTF-8"?>
                <Zusi>
                    <Info DateiTyp="Zug" Version="A.5" MinVersion="A.1"/>
                    <Zug Gattung="RB" Nummer="10001">
                        <Datei/>
                        <FahrzeugVarianten/>
                    </Zug>
                </Zusi>
            "#),
            ("user_dir/test.trn", r#"
                <?xml version="1.0" encoding="UTF-8"?>
                <Zusi>
                    <Info DateiTyp="Zug" Version="A.5" MinVersion="A.1"/>
                    <Zug Gattung="RE" Nummer="20002">
                        <Datei/>
                        <FahrzeugVarianten/>
                    </Zug>
                </Zusi>
            "#),
        ]).unwrap();

        let central_dir = tmp_dir_helper.path().join("central_dir");
        let user_dir = tmp_dir_helper.path().join("user_dir");
        let env = ZusiEnvironmentPath::new(&central_dir, &user_dir).with_path("test.trn").unwrap();

        let zug = env.read_xml_file::<TypedZusi::<Zug>>().unwrap().value;

        assert_eq!(
            zug,
            Zug::builder()
                .gattung("RE".into())
                .nummer("20002".into())
                .fahrplan_datei(Datei::builder().build())
                .fahrzeug_varianten(FahrzeugVarianten::builder().build())
                .build(),
        );
    }

    #[test]
    fn test_read_xml_file_that_does_not_exist_in_user_dir() {
        let tmp_dir_helper = TmpDirHelper::<ListInputValue>::from(vec![
            ("central_dir/test.trn", r#"
                <?xml version="1.0" encoding="UTF-8"?>
                <Zusi>
                    <Info DateiTyp="Zug" Version="A.5" MinVersion="A.1"/>
                    <Zug Gattung="RB" Nummer="10001">
                        <Datei/>
                        <FahrzeugVarianten/>
                    </Zug>
                </Zusi>
            "#),
            ("user_dir/testxyz.trn", r#"
                <?xml version="1.0" encoding="UTF-8"?>
                <Zusi>
                    <Info DateiTyp="Zug" Version="A.5" MinVersion="A.1"/>
                    <Zug Gattung="RE" Nummer="20002">
                        <Datei/>
                        <FahrzeugVarianten/>
                    </Zug>
                </Zusi>
            "#),
        ]).unwrap();

        let central_dir = tmp_dir_helper.path().join("central_dir");
        let user_dir = tmp_dir_helper.path().join("user_dir");
        let env = ZusiEnvironmentPath::new(&central_dir, &user_dir).with_path("test.trn").unwrap();

        let zug = env.read_xml_file::<TypedZusi::<Zug>>().unwrap().value;

        assert_eq!(
            zug,
            Zug::builder()
                .gattung("RB".into())
                .nummer("10001".into())
                .fahrplan_datei(Datei::builder().build())
                .fahrzeug_varianten(FahrzeugVarianten::builder().build())
                .build(),
        );
    }

    #[test]
    fn test_write_xml_file() {
        let tmp_dir_helper = TmpDirHelper::<ListInputValue>::from(vec![
            ("central_dir/test.trn", r#"
                <?xml version="1.0" encoding="UTF-8"?>
                <Zusi>
                    <Info DateiTyp="Zug" Version="A.5" MinVersion="A.1"/>
                    <Zug Gattung="RB" Nummer="10001">
                        <Datei/>
                        <FahrzeugVarianten/>
                    </Zug>
                </Zusi>
            "#),
            ("user_dir/testxyz.trn", r#"
                <?xml version="1.0" encoding="UTF-8"?>
                <Zusi>
                    <Info DateiTyp="Zug" Version="A.5" MinVersion="A.1"/>
                    <Zug Gattung="RE" Nummer="20002">
                        <Datei/>
                        <FahrzeugVarianten/>
                    </Zug>
                </Zusi>
            "#),
        ]).unwrap();

        let central_dir = tmp_dir_helper.path().join("central_dir");
        let user_dir = tmp_dir_helper.path().join("user_dir");
        let env = ZusiEnvironmentPath::new(&central_dir, &user_dir).with_path("test.trn").unwrap();

        let mut zug = env.read_xml_file::<TypedZusi::<Zug>>().unwrap();

        zug.value.gattung = "IRE".into();
        zug.value.nummer = "30003".into();

        env.write_xml_file::<Zusi>(zug.into(), true).unwrap();

        tmp_dir_helper.assert_with(vec![
            ("user_dir/test.trn", r#"
                <?xml version="1.0" encoding="UTF-8"?>
                <Zusi>
                    <Info DateiTyp="Zug" Version="A.5" MinVersion="A.1"/>
                    <Zug Gattung="IRE" Nummer="30003">
                        <Datei/>
                        <FahrzeugVarianten/>
                    </Zug>
                </Zusi>
            "#),
        ], true, |path_actual, (_, content_expected)| {
            assert_eq!(read_xml_file(path_actual), cleanup_xml(content_expected));
        });
    }
}