use super::{Holiday, NthWeek, Weekday};
use chrono::NaiveDate;

pub fn us_settlement_holidays() -> Vec<Holiday> {
    vec![
        // Saturdays
        Holiday::WeekDay(Weekday::Sat),
        // Sundays
        Holiday::WeekDay(Weekday::Sun),
        // New Year's day
        Holiday::ModifiedMovableYearlyDay {
            month: 1,
            day: 1,
            first: None,
            last: None,
        },
        // Martin Luther King's birthday (third Monday in January)
        Holiday::MonthWeekday {
            month: 1,
            weekday: Weekday::Mon,
            nth: NthWeek::Third,
            first: Some(1983),
            last: None,
        },
        // Washington's birthday (third Monday in February)
        Holiday::MonthWeekday {
            month: 2,
            weekday: Weekday::Mon,
            nth: NthWeek::Third,
            first: Some(1971),
            last: None,
        },
        Holiday::ModifiedMovableYearlyDay {
            month: 2,
            day: 22,
            first: None,
            last: Some(1970),
        },
        // Memorial Day (last Monday in May)
        Holiday::MonthWeekday {
            month: 5,
            weekday: Weekday::Mon,
            nth: NthWeek::Last,
            first: Some(1971),
            last: None,
        },
        Holiday::ModifiedMovableYearlyDay {
            month: 5,
            day: 30,
            first: None,
            last: Some(1970),
        },
        // Juneteenth (Monday if Sunday or Friday if Saturday)
        Holiday::ModifiedMovableYearlyDay {
            month: 6,
            day: 19,
            first: Some(2022),
            last: None,
        },
        // Independence Day (Monday if Sunday or Friday if Saturday)
        Holiday::ModifiedMovableYearlyDay {
            month: 7,
            day: 4,
            first: None,
            last: None,
        },
        // Labor Day (first Monday in September)
        Holiday::MonthWeekday {
            month: 9,
            weekday: Weekday::Mon,
            nth: NthWeek::First,
            first: None,
            last: None,
        },
        // Columbus Day (second Monday in October)
        Holiday::MonthWeekday {
            month: 10,
            weekday: Weekday::Mon,
            nth: NthWeek::Second,
            first: Some(1971),
            last: None,
        },
        // Veteran's Day
        Holiday::MonthWeekday {
            month: 10,
            weekday: Weekday::Mon,
            nth: NthWeek::Fourth,
            first: None,
            last: Some(1970),
        },
        Holiday::ModifiedMovableYearlyDay {
            month: 11,
            day: 11,
            first: Some(1971),
            last: Some(1977),
        },
        Holiday::MonthWeekday {
            month: 10,
            weekday: Weekday::Mon,
            nth: NthWeek::Fourth,
            first: Some(1978),
            last: None,
        },
        // Thanksgiving Day (fourth Thursday in November)
        Holiday::MonthWeekday {
            month: 11,
            weekday: Weekday::Thu,
            nth: NthWeek::Fourth,
            first: None,
            last: None,
        },
        // Christmas (Monday if Sunday or Friday if Saturday)
        Holiday::ModifiedMovableYearlyDay {
            month: 12,
            day: 25,
            first: None,
            last: None,
        },
    ]
}

pub fn uk_settlement_holidays() -> Vec<Holiday> {
    vec![
        // Saturdays
        Holiday::WeekDay(Weekday::Sat),
        // Sundays
        Holiday::WeekDay(Weekday::Sun),
        // New Year's day
        Holiday::MovableYearlyDay {
            month: 1,
            day: 1,
            first: None,
            last: None,
        },
        // Good Friday
        Holiday::EasterOffset {
            offset: -2,
            first: None,
            last: None,
        },
        // Easter Monday
        Holiday::EasterOffset {
            offset: 1,
            first: None,
            last: None,
        },
        // first Monday of May, moved two times in history to 8th of May
        Holiday::MonthWeekday {
            month: 5,
            weekday: Weekday::Mon,
            nth: NthWeek::First,
            first: None,
            last: Some(1994),
        },
        Holiday::SingularDay(NaiveDate::from_ymd(1995, 5, 8)),
        Holiday::MonthWeekday {
            month: 5,
            weekday: Weekday::Mon,
            nth: NthWeek::First,
            first: Some(1996),
            last: Some(2019),
        },
        Holiday::SingularDay(NaiveDate::from_ymd(2020, 5, 8)),
        Holiday::MonthWeekday {
            month: 5,
            weekday: Weekday::Mon,
            nth: NthWeek::First,
            first: Some(2021),
            last: None,
        },
        // last Monday of May (Spring Bank Holiday), has been skipped two times
        Holiday::MonthWeekday {
            month: 5,
            weekday: Weekday::Mon,
            nth: NthWeek::Last,
            first: None,
            last: Some(2001),
        },
        Holiday::MonthWeekday {
            month: 5,
            weekday: Weekday::Mon,
            nth: NthWeek::Last,
            first: Some(2003),
            last: Some(2011),
        },
        Holiday::MonthWeekday {
            month: 5,
            weekday: Weekday::Mon,
            nth: NthWeek::Last,
            first: Some(2013),
            last: None,
        },
        // last Monday of August (Summer Bank Holiday)
        Holiday::MonthWeekday {
            month: 8,
            weekday: Weekday::Mon,
            nth: NthWeek::Last,
            first: None,
            last: None,
        },
        // Christmas
        Holiday::MovableYearlyDay {
            month: 12,
            day: 25,
            first: None,
            last: None,
        },
        // Boxing Day
        Holiday::MovableYearlyDay {
            month: 12,
            day: 26,
            first: None,
            last: None,
        },
        // Golden Jubilee
        Holiday::SingularDay(NaiveDate::from_ymd(2002, 6, 3)),
        // Special Spring Holiday
        Holiday::SingularDay(NaiveDate::from_ymd(2002, 6, 4)),
        // Royal Wedding
        Holiday::SingularDay(NaiveDate::from_ymd(2011, 4, 29)),
        // Diamond Jubilee
        Holiday::SingularDay(NaiveDate::from_ymd(2012, 6, 4)),
        // Special Spring Holiday
        Holiday::SingularDay(NaiveDate::from_ymd(2012, 6, 5)),
        // Introduction of EUR
        Holiday::SingularDay(NaiveDate::from_ymd(1999, 12, 31)),
    ]
}

pub fn target_holidays() -> Vec<Holiday> {
    vec![
        // Saturdays
        Holiday::WeekDay(Weekday::Sat),
        // Sundays
        Holiday::WeekDay(Weekday::Sun),
        // New Year's day
        Holiday::YearlyDay {
            month: 1,
            day: 1,
            first: None,
            last: None,
        },
        // Good Friday
        Holiday::EasterOffset {
            offset: -2,
            first: Some(2000),
            last: None,
        },
        // Easter Monday
        Holiday::EasterOffset {
            offset: 1,
            first: Some(2000),
            last: None,
        },
        // Labour Day
        Holiday::YearlyDay {
            month: 5,
            day: 1,
            first: Some(2000),
            last: None,
        },
        // Christmas
        Holiday::YearlyDay {
            month: 12,
            day: 25,
            first: None,
            last: None,
        },
        // Day of Goodwill
        Holiday::YearlyDay {
            month: 12,
            day: 26,
            first: Some(2000),
            last: None,
        },
        // December 31st, 1998, 1999, and 2001 only
        Holiday::SingularDay(NaiveDate::from_ymd(1998, 12, 31)),
        Holiday::SingularDay(NaiveDate::from_ymd(1999, 12, 31)),
        Holiday::SingularDay(NaiveDate::from_ymd(2001, 12, 31)),
        Holiday::SingularDay(NaiveDate::from_ymd(1995, 5, 8)),
    ]
}

pub fn nok_holidays() -> Vec<Holiday> {
    vec![
        // Saturdays
        Holiday::WeekDay(Weekday::Sat),
        // Sundays
        Holiday::WeekDay(Weekday::Sun),
        // Holy Thursday
        Holiday::EasterOffset {
            offset: -4,
            first: None,
            last: None,
        },
        // Good Friday
        Holiday::EasterOffset {
            offset: -3,
            first: None,
            last: None,
        },
        // Easter Monday
        Holiday::EasterOffset {
            offset: 0,
            first: None,
            last: None,
        },
        // Ascension Thursday
        Holiday::EasterOffset {
            offset: 38,
            first: None,
            last: None,
        },
        // Whit Monday
        Holiday::EasterOffset {
            offset: 49,
            first: None,
            last: None,
        },
        // New Year's Day
        Holiday::YearlyDay {
            month: 1,
            day: 1,
            first: None,
            last: None,
        },
        // May Day
        Holiday::YearlyDay {
            month: 5,
            day: 1,
            first: None,
            last: None,
        },
        // National Independence Day
        Holiday::YearlyDay {
            month: 5,
            day: 17,
            first: None,
            last: None,
        },
        // Christmas Eve
        Holiday::YearlyDay {
            month: 12,
            day: 24,
            first: Some(2002),
            last: None,
        },
        // Christmas
        Holiday::YearlyDay {
            month: 12,
            day: 25,
            first: None,
            last: None,
        },
        // Boxing Day
        Holiday::YearlyDay {
            month: 12,
            day: 26,
            first: None,
            last: None,
        },
    ]
}
