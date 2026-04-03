#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Sort {
    #[default]
    Asc,
    Desc,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Currency {
    #[default]
    Usd,
}
