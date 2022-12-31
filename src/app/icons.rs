use std::fmt::Display;

pub enum Icon {
    New,
    Seen,
    Learnt,
}

impl From<i64> for Icon {
    fn from(count: i64) -> Self{
        match count {
            0 => Self::New,
            1..=4 => Self::Seen,
            5.. => Self::Learnt,
            i64::MIN..=-1_i64 => todo!()
        }
    }
}

impl Display for Icon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::New       => "",
            Self::Seen      => "\u{e22f}",
            Self::Learnt    => "\u{e21c}",
        })
    }
}