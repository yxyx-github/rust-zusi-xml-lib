use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use crate::xml::zusi::buchfahrplan::fahrplan_zeile::fahrplan_v_max_reduzierungen::ereignis::Ereignis;
use crate::xml::zusi::lib::abhaengigkeit::Abhaengigkeit;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct FahrplanVorgang {
    #[serde(rename = "@Beschreibung")]
    #[serde(default)]
    #[builder(default)]
    pub beschreibung: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum FahrplanVorgangValue {
    #[serde(rename = "Abhaengigkeit")]
    Abhaengigkeit(Abhaengigkeit),

    #[serde(rename = "Ereignis")]
    Ereignis(Ereignis),
}
