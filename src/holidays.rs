use chrono::NaiveDate;
use chrono::Duration;

pub fn compute_holidays(year: i32) -> Holidays {
    let easter = easter_sunday(year);

    Holidays {
        new_years_day: NaiveDate::from_ymd_opt(year, 1, 1).unwrap(),
        epiphany: NaiveDate::from_ymd_opt(year, 1, 6).unwrap(),
        womens_day: NaiveDate::from_ymd_opt(year, 3, 8).unwrap(),
        good_friday: add_days(easter, -2),
        easter_sunday: easter,
        easter_monday: add_days(easter, 1),
        mardi_gras: add_days(easter, -47),
        ash_wednesday: add_days(easter, -46),
        ascension_day: add_days(easter, 39),
    }
}

// See https://en.wikipedia.org/wiki/Computus
fn compute_m_and_n(year: i32) -> (i32, i32) {
    assert!(year_in_range(year));
    let k = year / 100;
    let q = year / 400;
    let p = (13 + 8 * k) / 25;
    let n = (4 + k - q) % 7;
    let m = 15 + k - q - p;

    (m, n)
}

pub fn easter_sunday(year: i32) -> NaiveDate {
    assert!(year_in_range(year));

    let (m, n) = compute_m_and_n(year);

    let a = year % 4;
    let b = year % 7;
    let c = year % 19;
    let d = (19*c + m) % 30;
    let e = (2*a + 4*b + 6*d + n) % 7;
    let f = (c + 11*d + 22*e) / 451;
    let f1 = (22 + d + e - 7 * f) as u32;

    if f1 > 31 {
        // Easter sunday is in April
        let month = 4;
        let day: u32 = (d + e - 7*f - 9) as u32;
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    } else {
        // Easter sunday is in March
        let month = 3;
        let day = f1;
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }
}

fn add_days(date: NaiveDate, offset: i64) -> NaiveDate {
    date + Duration::days(offset)
}

fn year_in_range(year: i32) -> bool {
    year >= 1583 && year <= 8202
}

#[derive(Debug)]
pub struct Holidays {
    new_years_day: NaiveDate,
    epiphany: NaiveDate,
    womens_day: NaiveDate,
    good_friday: NaiveDate,
    easter_sunday: NaiveDate,
    easter_monday: NaiveDate,
    mardi_gras: NaiveDate,    // Faschingsdienstag
    ash_wednesday: NaiveDate,
    ascension_day: NaiveDate, // Christi Himmelfahrt
}


#[cfg(test)]
mod test {
    use chrono::NaiveDate;
    use crate::holidays::{compute_m_and_n, easter_sunday};

    #[test]
    fn test_compute_m_and_n() {
        assert_eq!((23, 3), compute_m_and_n(1777));
        assert_eq!((24, 5), compute_m_and_n(2001));
        assert_eq!((23, 3), compute_m_and_n(1700));
        assert_eq!((23, 4), compute_m_and_n(1800));
        assert_eq!((24, 5), compute_m_and_n(1900));
    }

    #[test]
    fn test_easter_sunday() {
        assert_eq!(NaiveDate::from_ymd(1583, 04, 10), easter_sunday(1583));
        assert_eq!(NaiveDate::from_ymd(1723, 03, 28), easter_sunday(1723));
        assert_eq!(NaiveDate::from_ymd(2017, 04, 16), easter_sunday(2017));
        assert_eq!(NaiveDate::from_ymd(2020, 04, 12), easter_sunday(2020));
        assert_eq!(NaiveDate::from_ymd(2021, 04, 04), easter_sunday(2021));
        assert_eq!(NaiveDate::from_ymd(8202, 04, 18), easter_sunday(8202));
    }

    #[test]
    fn test_easter_sunday_all() {
        for year in 1583..8202 {
            easter_sunday(year);
        }
    }

    #[test]
    fn test_holidays() {
        let holidays = compute_holidays(2020);
        assert_eq!(NaiveDate::from_ymd(2020, 04, 13), holidays.easter_monday);
    }

    #[test]
    fn test_add_days() {
        let today = NaiveDate::from_ymd(2020, 04, 28);
        let expected = NaiveDate::from_ymd(2020, 05, 02);
        assert_eq!(expected, add_days(today, 4));
    }
}
