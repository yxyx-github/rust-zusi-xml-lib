use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum AbhaengigkeitBedingung {
    #[serde(rename = "0")]
    Keine,

    #[serde(rename = "1")]
    WegGefahrenSeitAktivierungGroesserOderGleich, // m

    #[serde(rename = "2")]
    WegBisHauptsignalKleinerOderGleich, // m

    #[serde(rename = "3")]
    KmWertKleinerOderGleich, // km

    #[serde(rename = "4")]
    KmWertGroesserOderGleich, // km

    #[serde(rename = "5")]
    GefahreneZeitSeitAktivierungGroesserOderGleich, // s

    #[serde(rename = "6")]
    GeschwindigkeitKleinerOderGleich, // km/h

    #[serde(rename = "7")]
    GeschwindigkeitGroesserOderGleich, // km/h

    #[serde(rename = "8")]
    Zufallswert, // 0-100 %

    #[serde(rename = "9")]
    AndererZugIstNaeherAls, // m (Abstand Zugspitzen)

    #[serde(rename = "10")]
    AndererZugIstWeiterWegAls, // m (Abstand Zugspitzen)

    #[serde(rename = "11")]
    AndererZugGeschwindigkeitKleinerOderGleich, // km/h

    #[serde(rename = "12")]
    AndererZugGeschwindigkeitGroesserOderGleich, // km/h

    #[serde(rename = "13")]
    AndererZugLaeuftMehrHinterPlanAls, // s

    #[serde(rename = "14")]
    AndererZugLaeuftMehrVorPlanAls, // s

    #[serde(rename = "15")]
    ZugLaeuftMehrHinterPlanAls, // s

    #[serde(rename = "16")]
    ZugLaeuftMehrVorPlanAls, // s

    #[serde(rename = "17")]
    ZugIstanzeigegeführt,

    #[serde(rename = "18")]
    ZugIstInETCSBetriebsart,

    #[serde(rename = "19")]
    ZustimmungDurchExterneSoftware,
}

impl Default for AbhaengigkeitBedingung {
    fn default() -> Self {
        AbhaengigkeitBedingung::Keine
    }
}