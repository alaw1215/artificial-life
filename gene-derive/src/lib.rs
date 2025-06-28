extern crate proc_macro;

use std::marker::PhantomData;

use gene_traits::AminoAcid;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::{self, RngCore};
use syn::{Data, DeriveInput, Fields, Meta, Type, parse_macro_input};

fn get_promoter(type_id: u128, promoter_size: u8) -> Vec<AminoAcid> {
    let mut promoter: Vec<AminoAcid> = vec![];

    let seed = type_id;
    // Well.. I'd like a 128-bit seed, but 64 bits will do just fine
    let mut rng = StdRng::seed_from_u64(seed as u64);

    for _ in 0..promoter_size {
        let random_num: u64 = rng.next_u64();
        promoter.push(match random_num % 4 {
            0 => AminoAcid::A,
            1 => AminoAcid::C,
            2 => AminoAcid::T,
            _ => AminoAcid::G,
        });
    }

    promoter
}

#[proc_macro_derive(Gene, attributes(parser, promoter_size))]
pub fn derive_gene(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let parse_func: Vec<_> = input
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident("parser"))
        .map(|attr| match &attr.meta {
            Meta::List(meta) => meta.tokens.clone(),
            _ => proc_macro2::TokenStream::new(),
        })
        .collect();

    let promoter_size = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("promoter_size"))
        .and_then(|attr| match &attr.meta {
            Meta::NameValue(name_value) => match &name_value.value {
                syn::Expr::Lit(val) => match &val.lit {
                    syn::Lit::Int(int_val) => int_val.base10_parse::<u8>().ok().map(|i| i as u8),
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        });

    let parse_func = parse_func.first().unwrap();

    let id = input.ident;

    let id_str = id.to_string();

    let promoter = get_promoter(0, promoter_size.unwrap());
    let test_array = quote! { [ #(#promoter),* ] };
    let output = quote! {
        inventory::submit!{ GeneRegister { promoter: #test_array, type_str: #id_str, parser: #parse_func}}
    };

    output.into()
}
