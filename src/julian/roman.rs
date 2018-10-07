use super::{fixed_from_julian, julian_from_fixed, Julian, is_julian_leap_year};
use common::{alternate_divide, RD};

#[derive(Clone, Copy, Debug)]
pub struct Roman {
    pub year: i32,
    pub month: i32,
    pub event: Event,
    pub count: i32,
    pub leap: bool
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Event {
    Kalends,
    Nones,
    Ides
}


pub fn ides_of_month(month: i32) -> i32 {
    match month {
        3|5|7|10 => 15,
        _ => 13
    }
}

pub fn nones_of_month(month: i32) -> i32 {
    ides_of_month(month) - 8
}

pub fn fixed_from_roman(date: Roman) -> i32 {
    let start = match date.event {
        Event::Kalends => fixed_from_julian(Julian { year: date.year, month: date.month, day: 1 }),
        Event::Nones => fixed_from_julian(Julian { year: date.year, month: date.month, day: nones_of_month(date.month) }),
        Event::Ides => fixed_from_julian(Julian { year: date.year, month: date.month, day: ides_of_month(date.month) })
    };
    start - date.count
    + if is_julian_leap_year(date.year) && date.month == 3 && date.event == Event::Kalends {
        0
    } else {
        1
    }
    + (date.leap as i32)
}

pub fn roman_from_fixed(rd: RD) -> Roman {
    let Julian { year, month, day } = julian_from_fixed(rd);
    let next_month = alternate_divide(month + 1, 12).1;
    let adjusted_year = if next_month == 1 { year + 1 } else { year };
    if day == 1 {
        Roman { year, month, event: Event::Kalends, count: 1, leap: false }
    } else if day <= nones_of_month(month) {
        Roman { year, month, event: Event::Nones, count: nones_of_month(month) - day + 1, leap: false }
    } else if day <= ides_of_month(month) {
        Roman { year, month, event: Event::Ides, count: ides_of_month(month) - day + 1, leap: false }
    } else if month != 2 || !is_julian_leap_year(year) {
        let kalends = fixed_from_roman(Roman { year: adjusted_year, month: next_month, event: Event::Kalends, count: 1, leap: false });
        Roman { year: adjusted_year, month: next_month, event: Event::Kalends, count: kalends - rd + 1, leap: false }
    } else if day < 25 {
        Roman { year, month: 3, event: Event::Kalends, count: 30 - day, leap: false }
    } else {
        Roman { year, month: 3, event: Event::Kalends, count: 31 - day, leap: day == 25 }
    }
}
