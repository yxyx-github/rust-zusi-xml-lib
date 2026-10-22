use std::fmt::Debug;

use crate::xml::zusi::buchfahrplan::Buchfahrplan;
use crate::xml::zusi::fahrplan::Fahrplan;
use crate::xml::zusi::info::Info;
use crate::xml::zusi::result::ZusiResult;
use crate::xml::zusi::zug::Zug;
pub use quick_xml::DeError;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
pub use serde_helpers::xml::{FromXML, ToXML};
use thiserror::Error;
use crate::xml::zusi::strecke::Strecke;

pub mod lib;
pub mod info;
pub mod result;
pub mod zug;
pub mod fahrplan;
pub mod buchfahrplan;
pub mod strecke;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Zusi {
    #[serde(rename = "Info")]
    pub info: Info,

    #[serde(rename = "$value")]
    pub value: ZusiValue,
}

impl AsRef<Zusi> for Zusi {
    fn as_ref(&self) -> &Zusi {
        &self
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum ZusiValue {
    /// Version A.1
    #[serde(rename = "result")]
    Result(ZusiResult),

    /// Version A.6 => latest dev version
    #[serde(rename = "Fahrplan")]
    Fahrplan(Fahrplan),

    /// Version A.6 => latest dev version
    #[serde(rename = "Zug")]
    Zug(Zug),

    /// Version A.4 => latest dev version
    #[serde(rename = "Buchfahrplan")]
    Buchfahrplan(Vec<Buchfahrplan>),

    /// Version A.6
    #[serde(rename = "Strecke")]
    Strecke(Strecke),
}

#[derive(Deserialize, TypedBuilder, PartialEq, Debug, Clone)]
#[serde(deny_unknown_fields, rename = "Zusi")]
pub struct TypedZusi<T> {
    #[serde(rename = "Info")]
    pub info: Info,

    #[serde(rename = "$value")]
    pub value: T,
}

impl<T: ZusiValueType> AsRef<TypedZusi<T>> for TypedZusi<T> {
    fn as_ref(&self) -> &TypedZusi<T> {
        &self
    }
}

impl<T: ZusiValueType> From<TypedZusi<T>> for Zusi {
    fn from(value: TypedZusi<T>) -> Self {
        Zusi {
            info: value.info,
            value: value.value.into_zusi_value(),
        }
    }
}

pub trait ZusiValueType {
    fn into_zusi_value(self) -> ZusiValue;
}

#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[error("The value of the file is invalid.")]
pub struct InvalidValue;

impl ZusiValueType for ZusiResult {
    fn into_zusi_value(self) -> ZusiValue {
        ZusiValue::Result(self)
    }
}

impl TryFrom<Zusi> for TypedZusi<ZusiResult> {
    type Error = InvalidValue;

    fn try_from(value: Zusi) -> Result<Self, Self::Error> {
        if let ZusiValue::Result(variant) = value.value {
            Ok(Self {
                info: value.info,
                value: variant,
            })
        } else {
            Err(InvalidValue)
        }
    }
}

impl ZusiValueType for Fahrplan {
    fn into_zusi_value(self) -> ZusiValue {
        ZusiValue::Fahrplan(self)
    }
}

impl TryFrom<Zusi> for TypedZusi<Fahrplan> {
    type Error = InvalidValue;

    fn try_from(value: Zusi) -> Result<Self, Self::Error> {
        if let ZusiValue::Fahrplan(variant) = value.value {
            Ok(Self {
                info: value.info,
                value: variant,
            })
        } else {
            Err(InvalidValue)
        }
    }
}

impl ZusiValueType for Zug {
    fn into_zusi_value(self) -> ZusiValue {
        ZusiValue::Zug(self)
    }
}

impl TryFrom<Zusi> for TypedZusi<Zug> {
    type Error = InvalidValue;

    fn try_from(value: Zusi) -> Result<Self, Self::Error> {
        if let ZusiValue::Zug(variant) = value.value {
            Ok(Self {
                info: value.info,
                value: variant,
            })
        } else {
            Err(InvalidValue)
        }
    }
}

impl ZusiValueType for Buchfahrplan {
    fn into_zusi_value(self) -> ZusiValue {
        ZusiValue::Buchfahrplan(vec![self])
    }
}

impl ZusiValueType for Vec<Buchfahrplan> {
    fn into_zusi_value(self) -> ZusiValue {
        ZusiValue::Buchfahrplan(self)
    }
}

impl TryFrom<Zusi> for TypedZusi<Buchfahrplan> {
    type Error = InvalidValue;

    fn try_from(value: Zusi) -> Result<Self, Self::Error> {
        if let ZusiValue::Buchfahrplan(variant) = value.value {
            if variant.len() == 1 {
                Ok(Self {
                    info: value.info,
                    value: variant.first().unwrap().to_owned(),
                })
            }
            else {
                Err(InvalidValue)
            }
        } else {
            Err(InvalidValue)
        }
    }
}

impl TryFrom<Zusi> for TypedZusi<Vec<Buchfahrplan>> {
    type Error = InvalidValue;

    fn try_from(value: Zusi) -> Result<Self, Self::Error> {
        if let ZusiValue::Buchfahrplan(variant) = value.value {
            Ok(Self {
                info: value.info,
                value: variant,
            })
        } else {
            Err(InvalidValue)
        }
    }
}