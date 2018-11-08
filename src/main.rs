mod arithmetical;
mod common;

use arithmetical::*;

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
    {
        let date = islamic::islamic_from_fixed(fixed);
        assert_eq!(islamic::Islamic { year: 1364, month: 12, day: 6 }, date);
        assert_eq!(fixed, islamic::fixed_from_islamic(date));
    }
    {
        let date = hebrew::hebrew_from_fixed(fixed);
        assert_eq!(hebrew::Hebrew { year: 5706, month: 9, day: 7 }, date);
        assert_eq!(fixed, hebrew::fixed_from_hebrew(date));
    }
    {
        let date = hindu::solar::hindu_solar_from_fixed(fixed);
        assert_eq!(hindu::solar::HinduSolar { year: 5046, month: 7, day: 29 }, date);
        assert_eq!(fixed, hindu::solar::fixed_from_hindu_solar(date));
    }
    {
        let date = hindu::lunisolar::hindu_lunisolar_from_fixed(fixed);
        assert_eq!(hindu::lunisolar::HinduLunisolar { year: 5046, month: 8, leap_month: false, day: 8 }, date);
        assert_eq!(fixed, hindu::lunisolar::fixed_from_hindu_lunisolar(date));
    }
    {
        let date = mayan::long_count::long_count_from_fixed(fixed);
        assert_eq!(mayan::long_count::LongCount { baktun: 12, katun: 16, tun: 11, uinal: 16, kin: 9 }, date);
        assert_eq!(fixed, mayan::long_count::fixed_from_long_count(date));
    }
    {
        let date = mayan::haab::mayan_haab_from_fixed(fixed);
        assert_eq!(mayan::haab::Haab { month: 11, day: 7 }, date);
        assert_eq!(fixed, mayan::haab::mayan_haab_on_or_before(date, fixed));
    }
    {
        let date = mayan::tzolkin::mayan_tzolkin_from_fixed(fixed);
        assert_eq!(mayan::tzolkin::Tzolkin { number: 11, name: 9 }, date);
        assert_eq!(fixed, mayan::tzolkin::mayan_tzolkin_on_or_before(date, fixed));
    }
    {
        assert_eq!(Some(fixed), mayan::mayan_calendar_round_on_or_before(
            mayan::haab::Haab { month: 11, day: 7 },
            mayan::tzolkin::Tzolkin { number: 11, name: 9 },
            fixed
        ));
    }
    {
        let date = balinese_powukon::bali_powukon_from_fixed(fixed);
        assert_eq!(balinese_powukon::BaliPowukon { luang: true, dwiwara: 2, triwara: 1, caturwara: 1, pancawara: 3, sadwara: 1, saptawara: 2, asatawara: 5, sangawara: 7, dasawara: 2 }, date);
        assert_eq!(fixed, balinese_powukon::bali_on_or_before(date, fixed));
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
