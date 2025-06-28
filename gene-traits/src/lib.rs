use hashed_type_def::HashedTypeDef;
use quote::quote;
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AminoAcid {
    A,
    C,
    T,
    G,
}

impl AminoAcid {
    pub fn sample<R: Rng + ?Sized>(rng: &mut R) -> AminoAcid {
        match rng.random_range(0..=3) {
            0 => AminoAcid::A,
            1 => AminoAcid::C,
            2 => AminoAcid::T,
            _ => AminoAcid::G,
        }
    }
}

pub const fn get_promoter<const N: usize, T: HashedTypeDef>() -> [AminoAcid; N] {
    let mut arr = [AminoAcid::A; N];
    let hash = T::TYPE_HASH_NATIVE;
    let mut i = 0;
    while i < N {
        arr[i] = match hash & (0x3 << i) % 4 {
            0 => AminoAcid::A,
            1 => AminoAcid::C,
            2 => AminoAcid::T,
            _ => AminoAcid::G,
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

impl quote::ToTokens for AminoAcid {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(match self {
            AminoAcid::A => quote! { AminoAcid::A },
            AminoAcid::C => quote! { AminoAcid::C },
            AminoAcid::T => quote! { AminoAcid::T },
            AminoAcid::G => quote! { AminoAcid::G },
        });
    }
}
