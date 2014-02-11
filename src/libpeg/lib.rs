// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/*!
Generates an implementation of a Parsing Expression Grammar

Provides supports for generating parsers using the Rust language.
TODO: [Describe a PEG grammar]

# Examples
TODO: [Calculator example for the docs]

# References

* [Wikipedia: Parsing Expression Grammar](
    http://en.wikipedia.org/wiki/Parsing_expression_grammar)

*/

#[crate_id = "peg#0.10-pre"];
#[crate_type = "rlib"];
#[crate_type = "dylib"];
#[license = "MIT/ASL2"];
#[feature(macro_registrar)];

extern mod syntax;

use syntax::ast::{Name, TokenTree};
use syntax::codemap::Span;
use syntax::ext::base::{SyntaxExtension,
                        BasicMacroExpander,
                        NormalTT,
                        MacResult,
                        MRItem,
                        ExtCtxt};
use syntax::parse::token;
use syntax::parse::token::str_to_ident;`

#[macro_registrar]
#[doc(hidden)]
pub fn macro_registrar(register: |Name, SyntaxExtension|) {
    register(token::intern("parse_peg"),
             NormalTT(~BasicMacroExpander {
                expander: expand_peg,
                span: None
             },
             None));
}

#[allow(unused_variable)]
fn expand_peg(cx: &mut ExtCtxt, sp: Span, tts: &[TokenTree]) -> MacResult {
    let name = str_to_ident("test_name");
    let t = quote_item!(cx,
        pub mod $name {
            struct parser_name {
                id: uint
            }

            impl parser_name {
                pub fn parse(&self) -> uint {
                    self.id
                }
            }
        }
        );
    MRItem(t.unwrap())
}
