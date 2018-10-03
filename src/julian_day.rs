const EPOCH: f32 = -172142.5;
const MODIFIED_EPOCH: i32 = 678576;

pub fn moment_from_jd(jd: f32) -> f32 {
    jd + EPOCH
}

pub fn jd_from_moment(m: f32) -> f32 {
    m - EPOCH
}

pub fn fixed_from_jd(jd: f32) -> i32 {
    jd.floor() as i32
}

pub fn jd_from_fixed(rd: i32) -> f32 {
    jd_from_moment(rd as f32)
}

pub fn fixed_from_mjd(mjd: i32) -> i32 {
    mjd + MODIFIED_EPOCH
}

pub fn mjd_from_fixed(rd: i32) -> i32 {
    rd - MODIFIED_EPOCH
}