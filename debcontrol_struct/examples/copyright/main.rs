// SPDX-FileCopyrightText: 2022 Agathe Porte <microjoe@microjoe.org>
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

// This is a simplified example of a d/copyright format parser.
// It will only parse the copyright header described in
// https://www.debian.org/doc/packaging-manuals/copyright-format/1.0/

use debcontrol::{Paragraph, Field, parse_str};
use debcontrol_struct::DebControl;

#[derive(Debug, DebControl)]
pub struct CopyrightHeader {
    pub format: String,
    pub upstream_name: Option<String>,
    pub upstream_contact: Option<String>,
    pub source: Option<String>,
    pub disclaimer: Option<String>,
    pub comment: Option<String>,
    pub license: Option<String>,
    pub copyright: Option<String>,
}

fn main() {
    let input = include_str!("copyright");
    let paragraphs = parse_str(input).expect("could not parse");

    let data = CopyrightHeader::from_paragraph(&paragraphs[0]);
    println!("{:?}", data);
}