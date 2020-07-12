#[derive(Debug, PartialEq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergen {
    fn allergen_item_from_value(value: u32) -> Option<Allergen> {
        match value {
            1 => Some(Allergen::Eggs),
            2 => Some(Allergen::Peanuts),
            4 => Some(Allergen::Shellfish),
            8 => Some(Allergen::Strawberries),
            16 => Some(Allergen::Tomatoes),
            32 => Some(Allergen::Chocolate),
            64 => Some(Allergen::Pollen),
            128 => Some(Allergen::Cats),
            _ => None,
        }
    }
}

pub struct Allergies {
    score: u32,
}

impl Allergies {
    const MAX_ALLERGEN_SCORE: u32 = 256;
    pub fn new(score: u32) -> Self {
        Allergies { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergies().contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut normalized_score = self.score % Allergies::MAX_ALLERGEN_SCORE;
        let mut result: Vec<Allergen> = vec![];

        for i in (0..=7).rev() {
            let allergen_value = (2 as u32).pow(i);
            if allergen_value <= normalized_score {
                result.push(Allergen::allergen_item_from_value(allergen_value).unwrap());
                normalized_score -= allergen_value;
            }
        }

        result
    }
}
