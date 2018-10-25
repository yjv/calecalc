use common::{RD, Moment};

pub const EPOCH: JulianDay = -1721424.5;
pub const MODIFIED_EPOCH: RD = 678576;

type JulianDay = f64;
type ModifiedJulianDay = i32;
type Moment = f32;

pub fn moment_from_jd(day: JulianDay) -> Moment {
    day + EPOCH
}

pub fn jd_from_moment(moment: Moment) -> JulianDay {
    moment - EPOCH
}

pub fn fixed_from_jd(day: JulianDay) -> RD {
    moment_from_jd(day).floor() as RD
}

pub fn jd_from_fixed(date: RD) -> JulianDay {
    jd_from_moment(date as JulianDay)
}

pub fn fixed_from_mjd(day: ModifiedJulianDay) -> RD {
    day + MODIFIED_EPOCH
}

pub fn mjd_from_fixed(date: RD) -> ModifiedJulianDay {
    date - MODIFIED_EPOCH
}