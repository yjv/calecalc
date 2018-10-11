use common::{RD, divide, alternate_divide};
use gregorian::{gregorian_year_from_fixed, nth_kday, Gregorian};

pub const EPOCH: i32 = 1;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Iso {
    pub year: i32,
    pub week: i32,
    pub day: i32
}

pub fn fixed_from_iso(date: Iso) -> RD {
    nth_kday(date.week, 0, Gregorian { year: date.year - 1, month: 12, day: 28 }) + date.day
}

pub fn iso_from_fixed(date: RD) -> Iso {
    let approx = gregorian_year_from_fixed(date - 3).0;

    let year = if date >= fixed_from_iso(Iso { year: approx + 1, week: 1, day: 1 }) {
        approx + 1
    } else {
        approx
    };

    let week = divide(date - fixed_from_iso(Iso { year, week: 1, day: 1 }), 7).0 + 1;
    let day = alternate_divide(date, 7).1;

    Iso { year, week, day }
}
