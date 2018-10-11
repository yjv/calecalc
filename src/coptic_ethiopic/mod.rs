use common::divide;
use common::RD;
use gregorian::{fixed_from_gregorian, Gregorian};

pub mod coptic;
pub mod ethiopic;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CopticEthiopic {
    pub year: i32,
    pub month: i32,
    pub day: i32
}

fn is_leap_year(year: i32) -> bool {
    divide(year, 4).1 == 3
}

fn fixed_from_date(date: CopticEthiopic, epoch: RD) -> RD {
    epoch - 1
        + 365 * (date.year - 1)
        + divide(date.year, 4).0
        + 30 * (date.month - 1)
        + date.day
}

fn date_from_fixed(date: RD, epoch: RD) -> CopticEthiopic {
    let year = divide(4 * (date - epoch) + 1463, 1461).0;
    let month = divide(date - fixed_from_date(CopticEthiopic { year, month: 1, day: 1 }, epoch), 30).0 + 1;
    let day = date + 1 - fixed_from_date(CopticEthiopic { year, month, day: 1 }, epoch);
    CopticEthiopic { year, month, day }
}

fn date_in_gregorian(month: i32, day: i32, gregorian_year: i32, epoch: RD) -> Option<RD> {
    let jan1 = fixed_from_gregorian(Gregorian { year: gregorian_year, month: 1, day: 1 });
    let dec31 = fixed_from_gregorian(Gregorian { year: gregorian_year, month: 12, day: 31 });

    let y = date_from_fixed(jan1, epoch).year;

    [
        fixed_from_date(CopticEthiopic { year: y, month, day }, epoch),
        fixed_from_date(CopticEthiopic { year: y + 1, month, day }, epoch)
    ].iter().cloned().filter(|date| jan1 <= *date && *date <= dec31).next()
}
