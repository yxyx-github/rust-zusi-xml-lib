//! Library to serialize/deserialize xml files for [Zusi 3](https://www.zusi.de/) using Serde.
//!
//! It provides the necessary data structures for usage with Serde and a TryFrom implementation.

/// Includes all the required data structures. At the moment `.result.xml` files are supported only.
/// For details about the schema, please refer to the demo files included in the [Zusi](https://www.zusi.de/) installation.
pub mod xml;
pub mod delphi_timestamp;
mod serde_helpers;
