#[derive(Debug)]
pub struct Duration {
    value: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        let sec_in_year = 31_557_600 as f64;
        Duration {
            value: s as f64 / sec_in_year,
        }
    }
}

pub trait Planet {
    const COEF: f64;
    fn years_during(d: &Duration) -> f64 {
        (1.0 / Self::COEF) * d.value
    }
}

macro_rules! planets {
    ( $($x:ident - $y:expr), * ) => {
        $(
            pub struct $x;
            impl Planet for $x {
                const COEF: f64 = $y;
            }
        )*
    };
}

planets! {
    Mercury - 0.2408467,
    Venus - 0.61519726,
    Earth - 1.0,
    Mars - 1.8808158,
    Jupiter - 11.862615,
    Saturn - 29.447498,
    Uranus - 84.016846,
    Neptune - 164.79132
}
