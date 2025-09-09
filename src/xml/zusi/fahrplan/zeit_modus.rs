use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum ZeitModus {
    #[serde(rename = "0")]
    ZeitUndDatumGemaessFahrplanvorgabe,

    #[serde(rename = "1")]
    InsAktuelleJahrSchiebenMonatTagZeitGemaesFahrplanvorgabe,

    #[serde(rename = "2")]
    DatumAufAktuellenTagVerschieben,

    #[serde(rename = "3")]
    DatumUndZeitAufAktuellenTagUndZeitVerschieben,
}

impl Default for ZeitModus {
    fn default() -> Self {
        ZeitModus::ZeitUndDatumGemaessFahrplanvorgabe
    }
}
