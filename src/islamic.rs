use common::{divide, RD};
use gregorian::{fixed_from_gregorian, Gregorian};

pub const EPOCH: i32 = 227015;


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Islamic {
    pub year: i32,
    pub month: i32,
    pub day: i32
}

pub fn is_islamic_leap_year(year: i32) -> bool {
    divide(14 + 11 * year, 30).1 < 11
}

pub fn is_islamic_birashk_leap_year(year: i32) -> bool {
    divide(15 + 11 * year, 30).1 < 11
}

pub fn fixed_from_islamic(date: Islamic) -> RD {
    date.day
        + 29 * (date.month - 1)
        + divide(6 * date.month - 1, 11).0
        + (date.year - 1) * 354
        + divide(3 + 11 * date.year, 30).0
        + EPOCH - 1
}

pub fn fixed_from_islamic_birashk(date: Islamic) -> RD {
    date.day
        + 29 * (date.month - 1)
        + divide(6 * date.month - 1, 11).0
        + (date.year - 1) * 354
        + divide(4 + 11 * date.year, 30).0
        + EPOCH - 1
}

pub fn islamic_from_fixed(date: RD) -> Islamic {
    let year = divide(30 * (date - EPOCH) + 10646, 10631).0;
    let prior_days = date - fixed_from_islamic(Islamic { year, month: 1, day: 1 });
    let month = divide(11 * prior_days + 330, 325).0;
    let day = date - fixed_from_islamic(Islamic { year, month, day: 1 }) + 1;
    Islamic { year, month, day }
}

pub fn islamic_birashk_from_fixed(date: RD) -> Islamic {
    let year = divide(30 * (date - EPOCH) + 10645, 10631).0;
    let prior_days = date - fixed_from_islamic(Islamic { year, month: 1, day: 1 });
    let month = divide(11 * prior_days + 330, 325).0;
    let day = date - fixed_from_islamic(Islamic { year, month, day: 1 });
    Islamic { year, month, day }
}

pub fn islamic_in_gregorian(month: i32, day: i32, gregorian_year: i32) -> Vec<RD> {
    let jan1 = fixed_from_gregorian(Gregorian { year: gregorian_year, month: 1, day: 1 });
    let dec31 = fixed_from_gregorian(Gregorian { year: gregorian_year, month: 12, day: 31 });
    let year = islamic_from_fixed(jan1).year;
    [
        fixed_from_islamic(Islamic { year, month, day }),
        fixed_from_islamic(Islamic { year: year + 1, month, day }),
        fixed_from_islamic(Islamic { year: year + 2, month, day })
    ].iter().cloned().filter(|date| jan1 <= *date && *date <= dec31).collect()
}
