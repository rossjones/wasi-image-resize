use std::error::Error;

pub enum Signum {
    Positive,
    Negative,
}

impl From<i32> for Signum {
    fn from(value: i32) -> Self {
        if value < 0 {
            Signum::Positive
        } else {
            Signum::Negative
        }
    }
}

pub struct Scale(i32);

impl TryFrom<&mut String> for Scale {
    type Error = String;

    fn try_from(value: &mut String) -> Result<Self, Self::Error> {
        match value.chars().rev().nth(0) {
            Some('%') => {
                let s = Scale(value[0..value.len() - 1].parse::<i32>().unwrap_or_default());
                Ok(s)
            }
            _ => return Err("unknown unit".into()),
        }
    }
}

impl Scale {
    pub fn scale_values(&self, w: u32, h: u32) -> Result<(u32, u32), Box<dyn Error>> {
        let signum = Signum::from(self.0);
        let abs_scale = self.0.abs();

        let new_w = ((w as f32 / 100.0) * abs_scale as f32) as u32;
        let new_h = ((h as f32 / 100.0) * abs_scale as f32) as u32;

        match signum {
            Signum::Positive => Ok((w + new_w, h + new_h)),
            Signum::Negative => Ok((w - new_w, h - new_h)),
        }
    }
}
