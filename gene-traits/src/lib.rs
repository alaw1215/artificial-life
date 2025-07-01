use hashed_type_def::HashedTypeDef;
use quote::quote;
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Nucleotide {
    A,
    C,
    T,
    G,
}

impl Nucleotide {
    pub fn sample<R: Rng + ?Sized>(rng: &mut R) -> Nucleotide {
        match rng.random_range(0..=3) {
            0 => Nucleotide::A,
            1 => Nucleotide::C,
            2 => Nucleotide::T,
            _ => Nucleotide::G,
        }
    }
}

pub const fn get_promoter<const N: usize, T: HashedTypeDef>() -> [Nucleotide; N] {
    let mut arr = [Nucleotide::A; N];
    let hash = T::TYPE_HASH_NATIVE;
    let mut i = 0;
    while i < N {
        arr[i] = match hash & (0x3 << i) % 4 {
            0 => Nucleotide::A,
            1 => Nucleotide::C,
            2 => Nucleotide::T,
            _ => Nucleotide::G,
        };
        i += 1;
    }
    arr
}

#[macro_export]
macro_rules! register_gene {
    ($ty:ty,$parser:ident,$promoter_size:expr) => {
        inventory::submit!{ GeneRegister { promoter: get_promoter::<$promoter_size,$ty>(), type_str: stringify!($ty), parser: $parser}}
    };
    ($ty:ty,$parser:ident<$($base_pair_type:ty),*>,$promoter_size:expr) => {
        inventory::submit!{ GeneRegister { promoter: get_promoter::<$promoter_size,$ty>(), type_str: stringify!($ty), parser: $parser::<$($base_pair_type),*>}}
    }
}

impl quote::ToTokens for Nucleotide {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(match self {
            Nucleotide::A => quote! { AminoAcid::A },
            Nucleotide::C => quote! { AminoAcid::C },
            Nucleotide::T => quote! { AminoAcid::T },
            Nucleotide::G => quote! { AminoAcid::G },
        });
    }
}
