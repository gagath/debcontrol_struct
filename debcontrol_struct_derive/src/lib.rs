// SPDX-FileCopyrightText: 2022 Agathe Porte <microjoe@microjoe.org>
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Automatic Debian control file parsing for structs (proc macro).

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::Type;

use syn::{Data, DataStruct, DeriveInput, Fields};

// Verify if a type is an Option<...>
// From https://stackoverflow.com/a/55277337
fn is_option(typ: &Type) -> bool {
    match typ {
        Type::Path(typepath) if typepath.qself.is_none() => {
            let path = &typepath.path;
            path.segments.len() == 1 && path.segments.iter().next().unwrap().ident == "Option"
        }
        _ => false,
    }
}

// Parse an identifier to a DebControl field format
fn ident_to_parse(ident: &Ident) -> String {
    let input = ident.to_string();
    let mut out = String::with_capacity(input.len());
    let mut next_char_is_upper = true;

    for c in input.chars() {
        match c {
            '_' => {
                next_char_is_upper = true;
                out.push('-');
            }
            _ => {
                if next_char_is_upper {
                    for c in c.to_uppercase() {
                        out.push(c);
                    }
                    next_char_is_upper = false;
                } else {
                    out.push(c);
                }
            }
        }
    }

    out
}

fn impl_debcontrol_struct(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let fields = match &ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };

    // from_paragraph impl

    let field_name = fields.iter().map(|field| &field.ident);

    let vars = quote! {
        #(
            let mut #field_name = None;
        )*
    };

    let field_name = fields.iter().map(|field| &field.ident);
    let field_parse = fields
        .iter()
        .map(|field| &field.ident)
        .map(|i| i.as_ref().map(|x| ident_to_parse(x)));

    let matches = quote! {
        for field in &p.fields {
            match field.name {
                #(
                    #field_parse => {
                        #field_name = Some(field.value.clone());
                    }
                ),*
                _ => {}
            }
        }
    };

    let field_name = fields
        .iter()
        .filter(|field| !is_option(&field.ty))
        .map(|field| &field.ident);
    let field_parse = fields
        .iter()
        .map(|field| &field.ident)
        .map(|i| i.as_ref().map(|x| ident_to_parse(x)));

    let check = quote! {
        #(
            let #field_name = #field_name.ok_or(
                concat!(
                    "Could not find the mandatory \"",
                    #field_parse,
                    "\" field in paragraph"))?;
        )*
    };

    let field_name = fields.iter().map(|field| &field.ident);

    let from_paragraph = quote! {
        fn from_paragraph(p: &Paragraph) -> Result<Self, &'static str>
        {
            #vars
            #matches
            #check

            Ok(Self {
                #(
                    #field_name
                ),*
            })
        }
    };

    // to_paragraph impl

    let mandatory_fields_name = fields
        .iter()
        .filter(|field| !is_option(&field.ty))
        .map(|field| &field.ident);
    let mandatory_fields_parse = fields
        .iter()
        .filter(|field| !is_option(&field.ty))
        .map(|field| &field.ident)
        .map(|i| i.as_ref().map(|x| ident_to_parse(x)));

    let paragraph_decl = quote! {
        let mut p = Paragraph {
            fields: vec![
                #(Field {
                    name: #mandatory_fields_parse,
                    value: self.#mandatory_fields_name.clone(),
                }),*
            ]
        };
    };

    let optional_fields_name = fields
        .iter()
        .filter(|field| is_option(&field.ty))
        .map(|field| &field.ident);
    let optional_fields_parse = fields
        .iter()
        .filter(|field| is_option(&field.ty))
        .map(|field| &field.ident)
        .map(|i| i.as_ref().map(|x| ident_to_parse(x)));

    let optional_paragraphs = quote! {
        #(
            if let Some(f) = &self.#optional_fields_name {
                p.fields.push(Field {name: #optional_fields_parse, value: f.to_string()});
            }
        )*
    };

    let to_paragraph = quote! {
        fn to_paragraph(&self) -> Paragraph {
            #paragraph_decl
            #optional_paragraphs
            p
        }
    };

    let gen = quote! {
        impl DebControl for #name {
            #from_paragraph
            #to_paragraph
        }
    };

    gen.into()
}

#[proc_macro_derive(DebControl)]
pub fn derive_liststore_item(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_debcontrol_struct(&ast)
}

#[cfg(test)]
mod test {
    use super::*;
    use proc_macro2::Span;

    #[test]
    fn test_ident_to_parse() {
        assert_eq!(
            "Field",
            ident_to_parse(&Ident::new("field", Span::call_site()))
        );
        assert_eq!(
            "Composed-Field",
            ident_to_parse(&Ident::new("composed_field", Span::call_site()))
        );
        assert_eq!(
            "Already-Upper",
            ident_to_parse(&Ident::new("Already_Upper", Span::call_site()))
        );
    }
}
