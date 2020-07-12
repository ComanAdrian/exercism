use std::collections::HashMap;

pub struct CodonsInfo<'a> {
    records: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    fn new(records: HashMap<&'a str, &'a str>) -> Self {
        CodonsInfo { records }
    }

    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.records.get(codon).map(|&x| x)
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        let codons = rna
            .as_bytes()
            .chunks(3)
            .map(std::str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .unwrap();
        let proteins = codons
            .into_iter()
            .map(|codon| self.records.get(codon).map(|&x| x))
            .take_while(|protein| match protein {
                Some(p) => *p != "stop codon",
                None => false,
            })
            .inspect(|x| println!("made it through filter: {:?}", x))
            .collect::<Option<Vec<&'a str>>>();

        match proteins {
            Some(res) => {
                if res.is_empty() {
                    None
                } else {
                    Some(res)
                }
            }
            None => None,
        }
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    let mut records: HashMap<&'a str, &'a str> = HashMap::new();

    for (codon, protein) in pairs.iter() {
        records.insert(*codon, *protein);
    }
    CodonsInfo::new(records)
}
