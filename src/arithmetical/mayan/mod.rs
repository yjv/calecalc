pub mod long_count;
pub mod haab;
pub mod tzolkin;

use common::{RD, divide};

// julian day 9/6/-3114 julian
pub static EPOCH: RD = -1137142;

pub fn mayan_calendar_round_on_or_before(haab: haab::Haab, tzolkin: tzolkin::Tzolkin, date: RD) -> Option<RD> {
    let haab_count = haab::mayan_haab_ordinal(haab) + haab::EPOCH;
    let tzolkin_count = tzolkin::mayan_tzolkin_ordinal(tzolkin) + tzolkin::EPOCH;
    let diff = tzolkin_count - haab_count;

    if divide(diff, 5).1 == 0 {
        Some(date - divide(date - haab_count - 365 * diff, 18980).1)
    } else {
        None
    }
}
