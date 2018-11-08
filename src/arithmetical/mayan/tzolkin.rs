use common::{divide, alternate_divide, RD};

// mayan_tzolkin_ordinal(Tzolkin { number: 4, name: 20 }) == 159
pub static EPOCH: i32 = super::EPOCH - 159;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Tzolkin {
    pub number: i32,
    pub name: i32
}

pub fn mayan_tzolkin_ordinal(date: Tzolkin) -> i32 {
    divide(date.number - 1 + 39 * (date.number - date.name), 260).1
}

pub fn mayan_tzolkin_from_fixed(date: RD) -> Tzolkin {
    let count = date - EPOCH + 1;
    let number = alternate_divide(count, 13).1;
    let name = alternate_divide(count, 20).1;
    Tzolkin { number, name }
}

pub fn mayan_tzolkin_on_or_before(tzolkin: Tzolkin, date: RD) -> RD {
    date - divide(date - EPOCH - mayan_tzolkin_ordinal(tzolkin), 260).1
}
