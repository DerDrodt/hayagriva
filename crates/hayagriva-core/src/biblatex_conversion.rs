use biblatex::{self as tex, DateValue};

use crate::Date;

pub fn date(date: tex::Date) -> Date {
    let approximate = date.uncertain || date.approximate;

    match date.value {
        DateValue::At(x) | DateValue::After(x) | DateValue::Before(x) => Date {
            year: x.year,
            month: x.month,
            day: x.day,
            approximate,
            season: None,
        },
        DateValue::Between(_, x) => Date {
            year: x.year,
            month: x.month,
            day: x.day,
            approximate,
            season: None,
        },
    }
}
