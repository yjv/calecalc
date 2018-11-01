use common::{divide, RD};

// mayan_haab_ordinal(Haab { month: 18, day: 8 }) == 348
pub static EPOCH: i32 = super::EPOCH - 348;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Haab {
    pub month: i32,
    pub day: i32
}

pub fn mayan_haab_ordinal(date: Haab) -> i32 {
    (date.month - 1) * 20 + date.day
}

pub fn mayan_haab_from_fixed(date: RD) -> Haab {
    let count = divide(date - EPOCH, 365).1;
    let (elapsed_monrths, day) = divide(count, 20);
    Haab { month: elapsed_monrths + 1, day }
}

pub fn mayan_haab_on_or_before(haab: Haab, date: RD) -> RD {
    date - divide(date - EPOCH - mayan_haab_ordinal(haab), 365).1
}
