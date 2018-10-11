mod common;
mod julian_day;
mod gregorian;
mod julian;
mod coptic_ethiopic;
mod iso;

fn main() {
    let mut fixed = 710347;
    {
        {
            let date = gregorian::gregorian_from_fixed(fixed);
            assert_eq!(gregorian::Gregorian { year: 1945, month: 11, day: 12 }, date);
            assert_eq!(fixed, gregorian::fixed_from_gregorian(date));
        }
        {
            let date = gregorian::alt_gregorian_from_fixed(fixed);
            assert_eq!(gregorian::Gregorian { year: 1945, month: 11, day: 12 }, date);
            assert_eq!(fixed, gregorian::alt_fixed_from_gregorian(date));
        }
    }
    {
        let date = julian::julian_from_fixed(fixed);
        assert_eq!(julian::Julian { year: 1945, month: 10, day: 30 }, date);
        assert_eq!(fixed, julian::fixed_from_julian(date));
    }
    {
        let date = julian::roman::roman_from_fixed(fixed);
        assert_eq!(julian::roman::Roman { year: 1945, month: 11, event: julian::roman::Event::Kalends, count: 3, leap: false }, date);
        assert_eq!(fixed, julian::roman::fixed_from_roman(date));
    }
    {
        let date = julian_day::jd_from_fixed(fixed);
        assert_eq!(2431771.5, date);
        assert_eq!(fixed, julian_day::fixed_from_jd(date));
    }
    {
        let date = julian_day::mjd_from_fixed(fixed);
        assert_eq!(31771, date);
        assert_eq!(fixed, julian_day::fixed_from_mjd(date));
    }
    {
        let date = coptic_ethiopic::coptic::coptic_from_fixed(fixed);
        assert_eq!(coptic_ethiopic::CopticEthiopic { year: 1662, month: 3, day: 3 }, date);
        assert_eq!(fixed, coptic_ethiopic::coptic::fixed_from_coptic(date));
    }
    {
        let date = coptic_ethiopic::ethiopic::ethiopic_from_fixed(fixed);
        assert_eq!(coptic_ethiopic::CopticEthiopic { year: 1938, month: 3, day: 3 }, date);
        assert_eq!(fixed, coptic_ethiopic::ethiopic::fixed_from_ethiopic(date));
    }
    {
        let date = iso::iso_from_fixed(fixed);
        assert_eq!(iso::Iso { year: 1945, week: 46, day: 1 }, date);
        assert_eq!(fixed, iso::fixed_from_iso(date));
    }
//    let rd = gregorian::fixed_from_gregorian(gregorian::Gregorian {
//        year: 1899,
//        month: 12,
//        day: 31
//    });
//    let gregorian = gregorian::alt_gregorian_from_fixed(rd);
//    let i = common::cycles_of_days::day_of_week_from_fixed(rd);
//    println!("Hello, world! {}, {}, {:?}", rd, i, gregorian);
//    let julian1 = julian::julian_from_fixed(rd);
//    println!("{:?}, {}", julian1, julian::fixed_from_julian(julian1));
//    println!("{:?}", julian::julian_in_gregorian(12, 20, 1901).map(gregorian::gregorian_from_fixed));
}
