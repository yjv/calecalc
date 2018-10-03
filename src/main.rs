mod common;
mod julian_day;
mod gregorian;

fn main() {
    let rd = gregorian::alt_fixed_from_gregorian(gregorian::Gregorian {
        year: 1945,
        month: 11,
        day: 12
    });
    let gregorian = gregorian::alt_gregorian_from_fixed(rd);
    let i = common::cycles_of_days::day_of_week_from_fixed(rd);
    println!("Hello, world! {}, {}, {:?}", rd, i, gregorian);
}
