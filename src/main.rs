use time::macros::datetime;
#[cfg(feature = "builder")]
use zusi_xml_lib::xml::zusi::result::fahrt_eintrag::FahrtEintragBuilder;
use zusi_xml_lib::xml::zusi::result::fahrt_eintrag::FahrtTyp;

fn main() {
    println!("Hello, world!");

    #[cfg(feature = "builder")]
    let fahrt_eintrag = FahrtEintragBuilder::default().fahrt_zeit(datetime!(2019-01-01 23:14)).build().unwrap();
    println!("{fahrt_eintrag:?}");
}
