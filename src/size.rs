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

impl Size {
    fn as_str(&self) -> &str {
        match self {
            Size::Size3 => "3x3",
            Size::Size5 => "5x5",
            Size::Size7 => "7x7",
        }
    }

    pub(crate) fn as_usize(&self) -> usize {
        match self {
            Size::Size3 => 3,
            Size::Size5 => 5,
            Size::Size7 => 7,
        }
    }

    pub(crate) fn from_usize(raw: i32) -> Size {
        match raw {
            3 => Size::Size3,
            5 => Size::Size5,
            7 => Size::Size7,
            _ => panic!("Invalid size value"),
        }
    }
}
impl std::fmt::Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
