//! Automatic Debian control file parsing for structs.
//!
//! # Example
//!
//! ```rust
//! use debcontrol::{Paragraph, Field};
//! use debcontrol_struct::DebControl;
//!
//! #[derive(DebControl)]
//! struct DerivedStruct {
//!     first: String,
//!     multiple_words: String,
//!     optional: Option<String>,
//! }
//!
//! let input = &debcontrol::parse_str(
//!     "First: Hello\n\
//!      Multiple-Words: World\n"
//! ).unwrap()[0];
//!
//! let derived = DerivedStruct::from_paragraph(&input).unwrap();
//! assert_eq!("Hello", derived.first);
//! assert_eq!("World", derived.multiple_words);
//! assert_eq!(None, derived.optional);
//! ```

use debcontrol::Paragraph;

pub trait DebControl {
    fn from_paragraph(p: &Paragraph) -> Result<Self, &'static str>
    where
        Self: Sized;
}

// Re-export #[derive(DebControl)].
#[cfg(feature = "derive")]
#[doc(hidden)]
pub use debcontrol_struct_derive::DebControl;

#[cfg(test)]
mod manual {
    use crate::*;
    use debcontrol::{Field, Paragraph};

    macro_rules! mandatory {
        ( $x:expr ) => {{
            $x.ok_or(concat!(
                "Could not find the mandatory \"",
                stringify!($x),
                "\" field in paragraph"
            ))
        }};
    }

    struct StandaloneLicense {
        license: String,
        comment: Option<String>,
    }

    impl DebControl for StandaloneLicense {
        fn from_paragraph(p: &Paragraph) -> Result<Self, &'static str> {
            let mut license = None;
            let mut comment = None;

            for field in &p.fields {
                match field.name {
                    "License" => {
                        license = Some(field.value.clone());
                    }
                    "Comment" => {
                        comment = Some(field.value.clone());
                    }
                    _ => {}
                }
            }

            let license = mandatory!(license)?;
            Ok(StandaloneLicense { license, comment })
        }
    }

    #[test]
    fn test_parse_standalone_license() {
        let input = Paragraph {
            fields: vec![Field {
                name: "License",
                value: "Expat".into(),
            }],
        };

        let license = StandaloneLicense::from_paragraph(&input).unwrap();

        assert_eq!("Expat", license.license);
        assert_eq!(None, license.comment);
    }

    #[test]
    fn test_parse_standalone_license_extended() {
        let input = Paragraph {
            fields: vec![
                Field {
                    name: "License",
                    value: "Expat".into(),
                },
                Field {
                    name: "Comment",
                    value: "Curious license to use...".into(),
                },
            ],
        };

        let license = StandaloneLicense::from_paragraph(&input).unwrap();

        assert_eq!("Expat", license.license);
        assert_eq!("Curious license to use...", license.comment.unwrap());
    }

    #[test]
    fn test_parse_standalone_license_bogus() {
        let input = Paragraph {
            fields: vec![Field {
                name: "Lic",
                value: "Expat".into(),
            }],
        };

        assert!(StandaloneLicense::from_paragraph(&input).is_err());
    }
}

#[cfg(feature = "derive")]
#[cfg(test)]
mod derive_test {
    use crate::*;
    use debcontrol::{Field, Paragraph};
    use debcontrol_struct_derive::DebControl;

    #[derive(DebControl)]
    struct DerivedStruct {
        first: String,
        multiple_words: String,
        optional: Option<String>,
    }

    #[test]
    fn test_parse_derived() {
        let input = Paragraph {
            fields: vec![
                Field {
                    name: "First",
                    value: "Hello".into(),
                },
                Field {
                    name: "Multiple-Words",
                    value: "World".into(),
                },
            ],
        };

        let derived = DerivedStruct::from_paragraph(&input).unwrap();

        assert_eq!("Hello", derived.first);
        assert_eq!("World", derived.multiple_words);
        assert_eq!(None, derived.optional);
    }

    #[test]
    fn test_parse_derived_extended() {
        let input = Paragraph {
            fields: vec![
                Field {
                    name: "First",
                    value: "Hello".into(),
                },
                Field {
                    name: "Multiple-Words",
                    value: "World".into(),
                },
                Field {
                    name: "Optional",
                    value: "!".into(),
                },
            ],
        };

        let derived = DerivedStruct::from_paragraph(&input).unwrap();

        assert_eq!("Hello", derived.first);
        assert_eq!("World", derived.multiple_words);
        assert_eq!(Some("!".into()), derived.optional);
    }

    #[test]
    fn test_parse_derived_bogus() {
        let input = Paragraph {
            fields: vec![Field {
                name: "First",
                value: "Hello".into(),
            }],
        };

        assert!(DerivedStruct::from_paragraph(&input).is_err());
    }
}
