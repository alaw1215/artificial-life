// Ok, let's think

// Define amino acids
enum AminoAcid {
    A,
    C,
    T,
    G,
}

struct Genome {
    _3prime_strand: Vec<AminoAcid>, // We will not be manipulating individual amino acids, so this
    // will be ok.  However, a more comprehsensive methodology would likely have to track each
    // amino acid with entities
    _5prime_strand: Vec<AminoAcid>,
}

// First and foremost, each component needs to have its own gene structure
// Can this be done automatically through proc macros perhaps?

// Additionally, there has to be some sort of genetic 'delimiter' to ensure each gene is in fact
// separable from other genes
