use std::{num::NonZeroU16, str::FromStr};

pub enum Rotation {
    Left { amount: NonZeroU16 },
    Right { amount: NonZeroU16 },
}

impl FromStr for Rotation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let first = chars.next().ok_or(())?;
        let rest: String = chars.collect();
        let amount = NonZeroU16::new(rest.parse().map_err(|_| ())?).ok_or(())?;

        match first {
            'L' => Ok(Self::Left { amount }),
            'R' => Ok(Self::Right { amount }),
            _ => Err(()),
        }
    }
}
