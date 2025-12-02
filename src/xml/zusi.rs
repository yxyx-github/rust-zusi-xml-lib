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

pub mod lib;
pub mod info;
pub mod result;
pub mod zug;
pub mod fahrplan;
pub mod buchfahrplan;

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

    /// Version A.6
    #[serde(rename = "Fahrplan")]
    Fahrplan(Fahrplan),

    /// Version A.6
    #[serde(rename = "Zug")]
    Zug(Zug),

    /// Version A.4
    #[serde(rename = "Buchfahrplan")]
    Buchfahrplan(Buchfahrplan),
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

#[derive(Debug)]
pub struct WrongVariant;

impl ZusiValueType for ZusiResult {
    fn into_zusi_value(self) -> ZusiValue {
        ZusiValue::Result(self)
    }
}

impl TryFrom<Zusi> for TypedZusi<ZusiResult> {
    type Error = WrongVariant;

    fn try_from(value: Zusi) -> Result<Self, Self::Error> {
        if let ZusiValue::Result(variant) = value.value {
            Ok(Self {
                info: value.info,
                value: variant,
            })
        } else {
            Err(WrongVariant)
        }
    }
}

impl ZusiValueType for Fahrplan {
    fn into_zusi_value(self) -> ZusiValue {
        ZusiValue::Fahrplan(self)
    }
}

impl TryFrom<Zusi> for TypedZusi<Fahrplan> {
    type Error = WrongVariant;

    fn try_from(value: Zusi) -> Result<Self, Self::Error> {
        if let ZusiValue::Fahrplan(variant) = value.value {
            Ok(Self {
                info: value.info,
                value: variant,
            })
        } else {
            Err(WrongVariant)
        }
    }
}

impl ZusiValueType for Zug {
    fn into_zusi_value(self) -> ZusiValue {
        ZusiValue::Zug(self)
    }
}

impl TryFrom<Zusi> for TypedZusi<Zug> {
    type Error = WrongVariant;

    fn try_from(value: Zusi) -> Result<Self, Self::Error> {
        if let ZusiValue::Zug(variant) = value.value {
            Ok(Self {
                info: value.info,
                value: variant,
            })
        } else {
            Err(WrongVariant)
        }
    }
}

impl ZusiValueType for Buchfahrplan {
    fn into_zusi_value(self) -> ZusiValue {
        ZusiValue::Buchfahrplan(self)
    }
}

impl TryFrom<Zusi> for TypedZusi<Buchfahrplan> {
    type Error = WrongVariant;

    fn try_from(value: Zusi) -> Result<Self, Self::Error> {
        if let ZusiValue::Buchfahrplan(variant) = value.value {
            Ok(Self {
                info: value.info,
                value: variant,
            })
        } else {
            Err(WrongVariant)
        }
    }
}