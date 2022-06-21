#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Period {
    Min_1,
    Min_5,
    Min_15,
    Min_30,
    Min_60,
    Day,
    Week,
    Month,
    Year,
}

impl Period {
    #[inline]
    pub(crate) fn minutes(&self) -> u8 {
        match self {
            Period::Min_5 => 5,
            Period::Min_15 => 15,
            Period::Min_30 => 30,
            Period::Min_60 => 60,
            _ => unreachable!(),
        }
    }
}

pub enum Type {
    Normal,
    USOQ,
}
