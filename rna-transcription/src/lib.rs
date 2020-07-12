#[derive(Debug, PartialEq)]
pub struct DNA {
    nucleotides: String,
}

#[derive(Debug, PartialEq)]
pub struct RNA {
    nucleotides: String,
}

impl DNA {
    const NUCLEOTIDES: [char; 4] = ['G', 'C', 'T', 'A'];

    pub fn new(dna: &str) -> Result<DNA, usize> {
        for (i, c) in dna.chars().enumerate() {
            if !DNA::NUCLEOTIDES.contains(&c) {
                return Err(i);
            }
        }

        Ok(DNA {
            nucleotides: dna.to_string(),
        })
    }

    pub fn into_rna(self) -> RNA {
        let rna_nucleotides = self
            .nucleotides
            .chars()
            .map(|c| {
                let position = DNA::NUCLEOTIDES
                    .iter()
                    .position(|dna_c| dna_c == &c)
                    .unwrap();
                RNA::NUCLEOTIDES[position]
            })
            .collect::<String>();

        RNA::new(&rna_nucleotides).unwrap()
    }
}

impl RNA {
    const NUCLEOTIDES: [char; 4] = ['C', 'G', 'A', 'U'];

    pub fn new(rna: &str) -> Result<RNA, usize> {
        for (i, c) in rna.chars().enumerate() {
            if !RNA::NUCLEOTIDES.contains(&c) {
                return Err(i);
            }
        }

        Ok(RNA {
            nucleotides: rna.to_string(),
        })
    }
}
