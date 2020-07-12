use std::collections::HashMap;
const NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !NUCLEOTIDES.contains(&nucleotide) {
        return Err(nucleotide)
    }

    let mut result: usize = 0;

    for c in dna.chars() {
        println!("{:?}", c);
        if !NUCLEOTIDES.contains(&c) {
            return Err(c)
        }

        if c == nucleotide {
            result += 1;
        }
    }

    Ok(result)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut hashMap: HashMap<char, usize> = HashMap::new();

    for i in NUCLEOTIDES.to_vec() {
        hashMap.insert(i, 0);
    }

    for c in dna.chars() {
        if !NUCLEOTIDES.contains(&c) {
            return Err(c);
        }

        if let Some(x) = hashMap.get_mut(&c) {
            *x += 1;
        }
    }

    Ok(hashMap)
}
