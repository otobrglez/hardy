use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "kebab-case")]
pub enum Size {
    #[value(name = "3")]
    Size3,
    #[value(name = "5")]
    Size5,
    #[value(name = "7")]
    Size7,
}

use Size::*;

impl Size {
    fn as_str(&self) -> &str {
        match self {
            Size3 => "3x3",
            Size5 => "5x5",
            Size7 => "7x7",
        }
    }

    pub fn as_usize(&self) -> usize {
        match self {
            Size3 => 3,
            Size5 => 5,
            Size7 => 7,
        }
    }

    pub fn from_usize(raw: i32) -> Size {
        match raw {
            3 => Size3,
            5 => Size5,
            7 => Size7,
            _ => panic!("Invalid size value"),
        }
    }
}
impl std::fmt::Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
