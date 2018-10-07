mod common;
mod julian_day;
mod gregorian;
mod julian;

fn main() {
    let rd = gregorian::fixed_from_gregorian(gregorian::Gregorian {
        year: 1899,
        month: 12,
        day: 31
    });
    let gregorian = gregorian::gregorian_from_fixed(rd);
    let i = common::cycles_of_days::day_of_week_from_fixed(rd);
    println!("Hello, world! {}, {}, {:?}", rd, i, gregorian);
    let julian1 = julian::julian_from_fixed(rd);
    println!("{:?}, {}", julian1, julian::fixed_from_julian(julian1));
    println!("{:?}", julian::julian_in_gregorian(12, 20, 1901).map(gregorian::gregorian_from_fixed));
}
