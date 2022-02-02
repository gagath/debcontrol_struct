<!--
SPDX-FileCopyrightText: 2022 Agathe Porte <microjoe@microjoe.org>

SPDX-License-Identifier: Apache-2.0 OR MIT
-->

# debcontrol_struct

[![Build](https://github.com/MicroJoe/debcontrol_struct/actions/workflows/ci.yml/badge.svg)](https://github.com/MicroJoe/debcontrol_struct/actions/workflows/ci.yml)
[![Latest version](https://img.shields.io/crates/v/debcontrol_struct.svg)](https://crates.io/crates/debcontrol_struct)
[![Documentation](https://docs.rs/debcontrol_struct/badge.svg)](https://docs.rs/debcontrol_struct)
[![License](https://img.shields.io/crates/l/debcontrol_struct.svg)](https://crates.io/crates/debcontrol_struct)

Automatic Debian control file parsing for structs.

## Usage

In order to use this crate, you have to add the following dependencies into
your project's `Cargo.toml` file:

```toml
[dependencies]
debcontrol_struct = "0.3.1"
```

## Example

After the crate is installed, you can enjoy the `DebControl` derive!

By defining the following structure:

```rust
use debcontrol::{Paragraph, Field};
use debcontrol_struct::DebControl;

#[derive(DebControl)]
struct DerivedStruct {
    first: String,
    multiple_words: String,
    optional: Option<String>,
}
```

You can then automatically parse the structure from a debcontrol Paragraph:

```rust
let input = &debcontrol::parse_str(
    "First: Hello\n\
     Multiple-Words: World\n"
).unwrap()[0];

let derived = DerivedStruct::from_paragraph(&input).unwrap();
assert_eq!("Hello", derived.first);
assert_eq!("World", derived.multiple_words);
assert_eq!(None, derived.optional);
```

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT
license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
