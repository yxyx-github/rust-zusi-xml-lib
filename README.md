# zusi-xml-lib

Simple library to serialize/deserialize xml files for [Zusi 3](https://www.zusi.de/) using Serde.
It provides the necessary data structures for usage with Serde and a TryFrom implementation.
Uses [serde-helpers](https://github.com/yxyx-github/rust-serde-helpers) for simplification.
By now only the types `result`, `Fahrplan`, `Zug` and `Buchfahrplan` are supported.
For details about the schemas, please refer to the demo files included in the [Zusi](https://www.zusi.de/) installation.