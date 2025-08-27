use gene_traits::amino_acid::AminoAcid;

pub fn expr_from_amino_acids(sequence: &[AminoAcid]) -> String {
    sequence
        .windows(2)
        .enumerate()
        // chunk-like reading: pairs at indices (0,1), (2,3), ...
        .filter(|(idx, _)| idx % 2 == 0)
        .map(|tpl| tpl.1)
        .map(|acid: &[AminoAcid]| match acid {
            &[AminoAcid::A, AminoAcid::A] => "(",
            &[AminoAcid::A, AminoAcid::P] => ")",
            &[AminoAcid::A, AminoAcid::F] => "*",
            &[AminoAcid::A, AminoAcid::M] => "/",
            &[AminoAcid::A, AminoAcid::K] => "^",
            &[AminoAcid::A, AminoAcid::S] => "+",
            &[AminoAcid::A, AminoAcid::W] => "-",
            &[AminoAcid::A, AminoAcid::T] => "%",
            &[AminoAcid::A, AminoAcid::Y] => "<",
            &[AminoAcid::A, AminoAcid::V] => ">",
            &[AminoAcid::A, AminoAcid::L] => "==",
            &[AminoAcid::A, AminoAcid::H] => ">=",
            &[AminoAcid::A, AminoAcid::D] => "<=",
            &[AminoAcid::A, AminoAcid::N] => "!=",
            &[AminoAcid::A, AminoAcid::R] => "!",
            &[AminoAcid::A, AminoAcid::I] => "math::sin",
            &[AminoAcid::A, AminoAcid::C] => "math::cos",
            &[AminoAcid::A, AminoAcid::E] => "math::ln",
            &[AminoAcid::A, AminoAcid::Q] => "math::log",
            &[AminoAcid::A, AminoAcid::G] => "math::log2",
            &[AminoAcid::P, AminoAcid::A] => "0",
            &[AminoAcid::P, AminoAcid::P] => "1",
            &[AminoAcid::P, AminoAcid::F] => "2",
            &[AminoAcid::P, AminoAcid::M] => "3",
            &[AminoAcid::P, AminoAcid::K] => "4",
            &[AminoAcid::P, AminoAcid::S] => "5",
            &[AminoAcid::P, AminoAcid::W] => "6",
            &[AminoAcid::P, AminoAcid::T] => "7",
            &[AminoAcid::P, AminoAcid::Y] => "8",
            &[AminoAcid::P, AminoAcid::V] => "9",
            &[AminoAcid::F, AminoAcid::A] => "dopamine",
            &[AminoAcid::F, AminoAcid::P] => "serotonin",
            &[AminoAcid::F, AminoAcid::F] => "norepinephrine",
            _ => "",
        })
        .collect()
}

// Determine the slice end before the terminator/promoter marker
pub fn last_idx_before_promoter(sequence: &[AminoAcid], promoter_size: usize) -> usize {
    if let Some(last_idx) = sequence
        .windows(promoter_size)
        .position(|w| w.eq(&[AminoAcid::UNKNOWN; 4]))
    {
        last_idx
    } else {
        sequence.len() - 1
    }
}

#[cfg(test)]
mod tests {
    use super::{expr_from_amino_acids, last_idx_before_promoter};
    use evalexpr::{build_operator_tree, context_map, DefaultNumericTypes};
    use gene_traits::amino_acid::AminoAcid;

    #[test]
    fn expr_parser_maps_pairs_to_expected_expression() {
        // Target expression: 2*dopamine - 3*serotonin + norepinephrine
        // Encoded as amino acids: PFAFFAAWPMAFFPASFF
        let amino_acids = [
            AminoAcid::P,
            AminoAcid::F, // "2"
            AminoAcid::A,
            AminoAcid::F, // "*"
            AminoAcid::F,
            AminoAcid::A, // "dopamine"
            AminoAcid::A,
            AminoAcid::W, // "-"
            AminoAcid::P,
            AminoAcid::M, // "3"
            AminoAcid::A,
            AminoAcid::F, // "*"
            AminoAcid::F,
            AminoAcid::P, // "serotonin"
            AminoAcid::A,
            AminoAcid::S, // "+"
            AminoAcid::F,
            AminoAcid::F, // "norepinephrine"
        ];

        let expected = "2*dopamine-3*serotonin+norepinephrine";
        let actual = expr_from_amino_acids(&amino_acids);
        assert_eq!(expected, actual);

        // Ensure the expression compiles and evaluates correctly with a context
        let ctx: evalexpr::HashMapContext<DefaultNumericTypes> = context_map! {
            "dopamine" => int 1,
            "serotonin" => int 1,
            "norepinephrine" => int 1
        }
        .unwrap();

        let precompiled = build_operator_tree::<DefaultNumericTypes>(&actual).unwrap();
        assert_eq!(precompiled.eval_int_with_context(&ctx), Ok(0));
    }

    #[test]
    fn last_index_detects_promoter_window() {
        // Data (6) + promoter (4 UNKNOWN) + trailing data
        let sequence = [
            AminoAcid::P, // 0
            AminoAcid::F, // 1
            AminoAcid::A, // 2
            AminoAcid::F, // 3
            AminoAcid::F, // 4
            AminoAcid::A, // 5
            AminoAcid::UNKNOWN, // promoter start at index 6
            AminoAcid::UNKNOWN,
            AminoAcid::UNKNOWN,
            AminoAcid::UNKNOWN, // promoter end at index 9
            AminoAcid::P,
            AminoAcid::F,
        ];

        let idx = last_idx_before_promoter(&sequence, 4);
        assert_eq!(idx, 6);
        let expr_segment = &sequence[0..idx];
        assert_eq!(expr_segment.len(), 6);
    }

    #[test]
    fn last_index_without_promoter_returns_len_minus_one() {
        let sequence = [
            AminoAcid::P,
            AminoAcid::F,
            AminoAcid::A,
            AminoAcid::F,
            AminoAcid::F,
            AminoAcid::A,
        ];
        let idx = last_idx_before_promoter(&sequence, 4);
        assert_eq!(idx, sequence.len() - 1);
    }
}