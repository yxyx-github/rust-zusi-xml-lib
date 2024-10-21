pub mod ereignis;

use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use crate::xml::zusi::buchfahrplan::fahrplan_zeile::fahrplan_v_max_reduzierungen::ereignis::Ereignis;

#[derive(Serialize, Deserialize, TypedBuilder, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct FahrplanVMaxReduzierungen {
    #[serde(rename = "$value")]
    pub value: Vec<FahrplanVMaxReduzierungenValue>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum FahrplanVMaxReduzierungenValue {
    #[serde(rename = "Ereignis")]
    Ereignis(Ereignis),
}
