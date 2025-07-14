pub mod dna {
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
}

pub mod rna {
    use crate::dna;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Nucleotide {
        A,
        C,
        U,
        G,
    }

    impl From<dna::Nucleotide> for Nucleotide {
        fn from(value: dna::Nucleotide) -> Self {
            match value {
                dna::Nucleotide::A => Self::A,
                dna::Nucleotide::C => Self::C,
                dna::Nucleotide::T => Self::U,
                dna::Nucleotide::G => Self::G,
            }
        }
    }
}

pub mod amino_acid {
    use crate::rna;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum AminoAcid {
        A,
        R,
        N,
        D,
        C,
        Q,
        E,
        G,
        H,
        I,
        L,
        K,
        M,
        F,
        P,
        S,
        T,
        W,
        Y,
        V,
        UNKNOWN,
    }

    impl From<[rna::Nucleotide; 3]> for AminoAcid {
        fn from(value: [rna::Nucleotide; 3]) -> Self {
            match value {
                [rna::Nucleotide::A, rna::Nucleotide::A, rna::Nucleotide::A] => Self::A,
                [rna::Nucleotide::A, rna::Nucleotide::A, rna::Nucleotide::C] => Self::A,
                [rna::Nucleotide::A, rna::Nucleotide::A, rna::Nucleotide::U] => Self::UNKNOWN,
                [rna::Nucleotide::A, rna::Nucleotide::A, rna::Nucleotide::G] => Self::A,
                [rna::Nucleotide::A, rna::Nucleotide::C, rna::Nucleotide::A] => Self::R,
                [rna::Nucleotide::A, rna::Nucleotide::C, rna::Nucleotide::C] => Self::R,
                [rna::Nucleotide::A, rna::Nucleotide::C, rna::Nucleotide::U] => Self::R,
                [rna::Nucleotide::A, rna::Nucleotide::C, rna::Nucleotide::G] => Self::N,
                [rna::Nucleotide::A, rna::Nucleotide::U, rna::Nucleotide::A] => Self::N,
                [rna::Nucleotide::A, rna::Nucleotide::U, rna::Nucleotide::C] => Self::N,
                [rna::Nucleotide::A, rna::Nucleotide::U, rna::Nucleotide::U] => Self::UNKNOWN,
                [rna::Nucleotide::A, rna::Nucleotide::U, rna::Nucleotide::G] => Self::D,
                [rna::Nucleotide::A, rna::Nucleotide::G, rna::Nucleotide::A] => Self::D,
                [rna::Nucleotide::A, rna::Nucleotide::G, rna::Nucleotide::C] => Self::D,
                [rna::Nucleotide::A, rna::Nucleotide::G, rna::Nucleotide::U] => Self::C,
                [rna::Nucleotide::A, rna::Nucleotide::G, rna::Nucleotide::G] => Self::C,
                [rna::Nucleotide::C, rna::Nucleotide::A, rna::Nucleotide::A] => Self::C,
                [rna::Nucleotide::C, rna::Nucleotide::A, rna::Nucleotide::C] => Self::Q,
                [rna::Nucleotide::C, rna::Nucleotide::A, rna::Nucleotide::U] => Self::Q,
                [rna::Nucleotide::C, rna::Nucleotide::A, rna::Nucleotide::G] => Self::Q,
                [rna::Nucleotide::C, rna::Nucleotide::C, rna::Nucleotide::A] => Self::E,
                [rna::Nucleotide::C, rna::Nucleotide::C, rna::Nucleotide::C] => Self::E,
                [rna::Nucleotide::C, rna::Nucleotide::C, rna::Nucleotide::U] => Self::E,
                [rna::Nucleotide::C, rna::Nucleotide::C, rna::Nucleotide::G] => Self::G,
                [rna::Nucleotide::C, rna::Nucleotide::U, rna::Nucleotide::A] => Self::G,
                [rna::Nucleotide::C, rna::Nucleotide::U, rna::Nucleotide::C] => Self::G,
                [rna::Nucleotide::C, rna::Nucleotide::U, rna::Nucleotide::U] => Self::H,
                [rna::Nucleotide::C, rna::Nucleotide::U, rna::Nucleotide::G] => Self::H,
                [rna::Nucleotide::C, rna::Nucleotide::G, rna::Nucleotide::A] => Self::H,
                [rna::Nucleotide::C, rna::Nucleotide::G, rna::Nucleotide::C] => Self::I,
                [rna::Nucleotide::C, rna::Nucleotide::G, rna::Nucleotide::U] => Self::I,
                [rna::Nucleotide::C, rna::Nucleotide::G, rna::Nucleotide::G] => Self::I,
                [rna::Nucleotide::U, rna::Nucleotide::A, rna::Nucleotide::A] => Self::L,
                [rna::Nucleotide::U, rna::Nucleotide::A, rna::Nucleotide::C] => Self::L,
                [rna::Nucleotide::U, rna::Nucleotide::A, rna::Nucleotide::U] => Self::L,
                [rna::Nucleotide::U, rna::Nucleotide::A, rna::Nucleotide::G] => Self::K,
                [rna::Nucleotide::U, rna::Nucleotide::C, rna::Nucleotide::A] => Self::K,
                [rna::Nucleotide::U, rna::Nucleotide::C, rna::Nucleotide::C] => Self::K,
                [rna::Nucleotide::U, rna::Nucleotide::C, rna::Nucleotide::U] => Self::M,
                [rna::Nucleotide::U, rna::Nucleotide::C, rna::Nucleotide::G] => Self::M,
                [rna::Nucleotide::U, rna::Nucleotide::U, rna::Nucleotide::A] => Self::UNKNOWN,
                [rna::Nucleotide::U, rna::Nucleotide::U, rna::Nucleotide::C] => Self::M,
                [rna::Nucleotide::U, rna::Nucleotide::U, rna::Nucleotide::U] => Self::UNKNOWN,
                [rna::Nucleotide::U, rna::Nucleotide::U, rna::Nucleotide::G] => Self::F,
                [rna::Nucleotide::U, rna::Nucleotide::G, rna::Nucleotide::A] => Self::F,
                [rna::Nucleotide::U, rna::Nucleotide::G, rna::Nucleotide::C] => Self::F,
                [rna::Nucleotide::U, rna::Nucleotide::G, rna::Nucleotide::U] => Self::P,
                [rna::Nucleotide::U, rna::Nucleotide::G, rna::Nucleotide::G] => Self::P,
                [rna::Nucleotide::G, rna::Nucleotide::A, rna::Nucleotide::A] => Self::P,
                [rna::Nucleotide::G, rna::Nucleotide::A, rna::Nucleotide::C] => Self::S,
                [rna::Nucleotide::G, rna::Nucleotide::A, rna::Nucleotide::U] => Self::S,
                [rna::Nucleotide::G, rna::Nucleotide::A, rna::Nucleotide::G] => Self::S,
                [rna::Nucleotide::G, rna::Nucleotide::C, rna::Nucleotide::A] => Self::T,
                [rna::Nucleotide::G, rna::Nucleotide::C, rna::Nucleotide::C] => Self::T,
                [rna::Nucleotide::G, rna::Nucleotide::C, rna::Nucleotide::U] => Self::T,
                [rna::Nucleotide::G, rna::Nucleotide::C, rna::Nucleotide::G] => Self::W,
                [rna::Nucleotide::G, rna::Nucleotide::U, rna::Nucleotide::A] => Self::W,
                [rna::Nucleotide::G, rna::Nucleotide::U, rna::Nucleotide::C] => Self::W,
                [rna::Nucleotide::G, rna::Nucleotide::U, rna::Nucleotide::U] => Self::Y,
                [rna::Nucleotide::G, rna::Nucleotide::U, rna::Nucleotide::G] => Self::Y,
                [rna::Nucleotide::G, rna::Nucleotide::G, rna::Nucleotide::A] => Self::Y,
                [rna::Nucleotide::G, rna::Nucleotide::G, rna::Nucleotide::C] => Self::V,
                [rna::Nucleotide::G, rna::Nucleotide::G, rna::Nucleotide::U] => Self::V,
                [rna::Nucleotide::G, rna::Nucleotide::G, rna::Nucleotide::G] => Self::V,
            }
        }
    }

    impl Into<u8> for AminoAcid {
        fn into(self) -> u8 {
            match self {
                AminoAcid::A => 0,
                AminoAcid::R => 1,
                AminoAcid::N => 2,
                AminoAcid::D => 3,
                AminoAcid::C => 4,
                AminoAcid::Q => 5,
                AminoAcid::E => 6,
                AminoAcid::G => 7,
                AminoAcid::H => 8,
                AminoAcid::I => 9,
                AminoAcid::L => 10,
                AminoAcid::K => 11,
                AminoAcid::M => 12,
                AminoAcid::F => 13,
                AminoAcid::P => 14,
                AminoAcid::S => 15,
                AminoAcid::T => 16,
                AminoAcid::W => 17,
                AminoAcid::Y => 18,
                AminoAcid::V => 19,
                AminoAcid::UNKNOWN => 0,
            }
        }
    }
}
