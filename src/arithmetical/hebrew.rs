use common::{divide, divide_f, RD, cycles_of_days::{day_of_week_from_fixed, kday_before}};
use gregorian::{fixed_from_gregorian, gregorian_year_from_fixed, Gregorian};
use coptic_ethiopic::coptic::{coptic_in_gregorian, coptic_from_fixed};
use std::iter::Iterator;

pub const EPOCH: i32 = -1373427;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Hebrew {
    pub year: i32,
    pub month: i32,
    pub day: i32
}

pub fn is_hebrew_leap_year(year: i32) -> bool {
    divide(7 * year + 1, 19).1 < 7
}

pub fn last_month_of_hebrew_year(year: i32) -> i32 {
    if is_hebrew_leap_year(year) {
        13
    } else {
        12
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Molad {
    pub date: RD,
    pub hour: i32,
    pub minute: i32,
    pub chalakim: i32
}

pub fn molad(month: i32, year: i32) -> Molad {
    // 7 is tishrei
    let months_elapsed = month - 7 + divide(235 * year - 234, 19).0;
    let half_days_elapsed = divide(months_elapsed, 2);
    let chalakim_elapsed = divide(months_elapsed * 793 - 876 + 12960 * half_days_elapsed.1, 25920);
    let elapsed_days = months_elapsed * 29 + half_days_elapsed.0 + chalakim_elapsed.0;
    let hours = divide(chalakim_elapsed.1, 1080);
    let minutes = divide(hours.1, 18);

    Molad { date: EPOCH + elapsed_days, hour: hours.0, minute: minutes.0, chalakim: minutes.1 }
}

pub fn hebrew_calendar_elapsed_days(year: i32) -> i32 {
    let molad = molad(7, year);
    let day = if molad.hour >= 12 {
        molad.date + 1
    } else {
        molad.date
    } - EPOCH;

    if divide(3 * (day + 1), 7).1 < 3 {
        day + 1
    } else {
        day
    }
}

pub fn hebrew_new_year_delay(year: i32) -> i32 {
    let ny0 = hebrew_calendar_elapsed_days(year - 1);
    let ny1 = hebrew_calendar_elapsed_days(year);
    let ny2 = hebrew_calendar_elapsed_days(year + 1);

    if ny2 - ny1 == 356 {
        2
    } else if ny1 - ny0 == 382 {
        1
    } else {
        0
    }
}

pub fn hebrew_new_year(year: i32) -> RD {
    EPOCH + hebrew_calendar_elapsed_days(year) + hebrew_new_year_delay(year)
}

pub fn last_day_of_hebrew_month(month: i32, year: i32) -> RD {
    match month {
        2|4|6|10|13 => 29,
        12 if is_hebrew_leap_year(year) => 29,
        8 if !is_long_marcheshvan(year) => 29,
        9 if is_short_kislev(year) => 29,
        _ => 30
    }
}

pub fn is_long_marcheshvan(year: i32) -> bool {
    match days_in_hebrew_year(year) {
        355|385 => true,
        _ => false
    }
}

pub fn is_short_kislev(year: i32) -> bool {
    match days_in_hebrew_year(year) {
        353|383 => true,
        _ => false
    }
}

pub fn days_in_hebrew_year(year: i32) -> i32 {
    hebrew_new_year(year + 1) - hebrew_new_year(year)
}

pub fn fixed_from_hebrew(date: Hebrew) -> RD {
    hebrew_new_year(date.year)
        + date.day - 1
        + if date.month < 7 {
            (7..last_month_of_hebrew_year(date.year) + 1).map(|month| last_day_of_hebrew_month(month, date.year)).sum::<i32>()
                + (1..date.month).map(|month| last_day_of_hebrew_month(month, date.year)).sum::<i32>()
        } else {
            (7..date.month).map(|month| last_day_of_hebrew_month(month, date.year)).sum::<i32>()
        }
}

pub fn hebrew_from_fixed(date: RD) -> Hebrew {
    // 35975351.0/98496.0 is the average length of a hebrew year
    let approx_year = divide_f(date as f64 - EPOCH as f64, 35975351.0/98496.0).0 as i32 + 1;

    // because years have irregular size the min year can be up to 2 off
    let year = (approx_year - 1..=approx_year + 1)
        .filter(|&year| hebrew_new_year(year) <= date)
        .last()
        .expect("Should always have a value")
    ;

    let start_month = if date < fixed_from_hebrew(Hebrew { year, month: 1, day: 1 }) {
        7
    } else {
        1
    };

    let month = (start_month..)
        .filter(|&month| fixed_from_hebrew(Hebrew { year, month, day: last_day_of_hebrew_month(month, year) }) >= date )
        .next()
        .expect("Should always have a value")
    ;

    let day = date - fixed_from_hebrew(Hebrew { year, month, day: 1}) + 1;
    Hebrew { year, month, day }
}

pub fn yom_kippur(gregorian_year: i32) -> RD {
    let year = gregorian_year - gregorian_year_from_fixed(EPOCH).0 + 1;
    fixed_from_hebrew(Hebrew { year, month: 7, day: 10 })
}

pub fn passover(gregorian_year: i32) -> RD {
    let year = gregorian_year - gregorian_year_from_fixed(EPOCH).0;
    fixed_from_hebrew(Hebrew { year, month: 1, day: 15 })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Omer {
    pub week: i32,
    pub day: i32
}

pub fn omer(date: RD) -> Option<Omer> {
    let c = date - passover(gregorian_year_from_fixed(date).0);

    if 1 <= c && c <= 49 {
        let (week, day) = divide(c, 7);
        Some(Omer { week, day })
    } else {
        None
    }
}

pub fn purim(gregorian_year: i32) -> RD {
    let year = gregorian_year - gregorian_year_from_fixed(EPOCH).0;
    fixed_from_hebrew(Hebrew { year, month: last_month_of_hebrew_year(year), day: 14 })
}

pub fn taanit_esther(gregorian_year: i32) -> RD {
    let purim_date = purim(gregorian_year);
    purim_date - if day_of_week_from_fixed(purim_date) == 0 { 3 } else { 1 }
}

pub fn tisha_bav(gregorian_year: i32) -> RD {
    let year = gregorian_year - gregorian_year_from_fixed(EPOCH).0;
    let av9 = fixed_from_hebrew(Hebrew { year, month: 5, day: 9 });

    if day_of_week_from_fixed(av9) == 6 {
        av9 + 1
    } else {
        av9
    }
}

pub fn hebrew_in_gregorian(month: i32, day: i32, gregorian_year: i32) -> Vec<RD> {
    let jan1 = fixed_from_gregorian(Gregorian { year: gregorian_year, month: 1, day: 1 });
    let dec31 = fixed_from_gregorian(Gregorian { year: gregorian_year, month: 12, day: 31 });
    let year = hebrew_from_fixed(jan1).year;
    [
        Hebrew { year, month, day },
        Hebrew { year: year + 1, month, day },
    ].iter().cloned().map(fixed_from_hebrew).filter(|&date| jan1 <= date && date <= dec31).collect()
}

pub fn tzom_tevet(gregorian_year: i32) -> Vec<RD> {
    hebrew_in_gregorian(10, 10, gregorian_year)
}

pub fn yom_hashoa(gregorian_year: i32) -> RD {
    let year = gregorian_year - gregorian_year_from_fixed(EPOCH).0;
    let nissan27 = fixed_from_hebrew(Hebrew { year, month: 1, day: 27 });

    if day_of_week_from_fixed(nissan27) == 0 {
        nissan27 + 1
    } else {
        nissan27
    }
}

pub fn yom_hazikaron(gregorian_year: i32) -> RD {
    let year = gregorian_year - gregorian_year_from_fixed(EPOCH).0;
    let iyyar4 = fixed_from_hebrew(Hebrew { year, month: 2, day: 4 });

    if day_of_week_from_fixed(iyyar4) > 3 {
        kday_before(iyyar4, 3)
    } else {
        iyyar4
    }
}

/// when we start saying vsen tal umatar
pub fn sheela(gregorian_year: i32) -> RD {
    coptic_in_gregorian(3, 26, gregorian_year).expect("This should always be a date since its not close enough to the ends")
}

pub fn birkath_hachama(gregorian_year: i32) -> Option<RD> {
    coptic_in_gregorian(7, 30, gregorian_year).filter(|&date| divide(coptic_from_fixed(date).year, 28).1 == 17)
}

pub fn hebrew_birthday(birth_date: Hebrew, year: i32) -> RD {
    if last_month_of_hebrew_year(birth_date.year) == birth_date.month {
        fixed_from_hebrew(Hebrew { year, month: last_month_of_hebrew_year(year), day: birth_date.day })
    } else {
        // the reason the day is not input correctly is to account for cheshvan and kislev which may
        // take it to the next month and wouldnt be valid for this year
        fixed_from_hebrew(Hebrew { year, month: birth_date.month, day: 1 }) + birth_date.day - 1
    }
}

pub fn hebrew_birthday_in_gregorian(birth_date: Hebrew, gregorian_year: i32) -> Vec<RD> {
    let jan1 = fixed_from_gregorian(Gregorian { year: gregorian_year, month: 1, day: 1 });
    let dec31 = fixed_from_gregorian(Gregorian { year: gregorian_year, month: 12, day: 31 });
    let year = hebrew_from_fixed(jan1).year;
    [
        hebrew_birthday(birth_date, year),
        hebrew_birthday(birth_date, year + 1)
    ].iter().cloned().filter(|&date| jan1 <= date && date <= dec31).collect()
}

/// based on talmudic encyclopedia (encyclopedia talmudis)
pub fn yahrtzeit(death_date: Hebrew, year: i32) -> RD {
    let (month, day) = if death_date.month == 8 && death_date.day == 30 && !is_long_marcheshvan(death_date.year + 1) {
        (9, 1)
    } else if death_date.month == 9 && death_date.day == 30 && is_short_kislev(death_date.year + 1) {
        (10, 1)
    } else if death_date.month == 13 {
        (last_month_of_hebrew_year(year), death_date.day)
    } else if death_date.month == 12 && death_date.day == 30 && !is_hebrew_leap_year(year) {
        (11, death_date.day)
    } else {
        (death_date.month, death_date.day)
    };
    fixed_from_hebrew(Hebrew { year, month, day })
}

pub fn yahrtzeit_in_gregorian(death_date: Hebrew, gregorian_year: i32) -> Vec<RD> {
    let jan1 = fixed_from_gregorian(Gregorian { year: gregorian_year, month: 1, day: 1 });
    let dec31 = fixed_from_gregorian(Gregorian { year: gregorian_year, month: 12, day: 31 });
    let year = hebrew_from_fixed(jan1).year;
    [
        yahrtzeit(death_date, year),
        yahrtzeit(death_date, year + 1)
    ].iter().cloned().filter(|&date| jan1 <= date && date <= dec31).collect()
}
