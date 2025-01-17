//! Formatting helpers for a `Date`.

#![allow(non_snake_case)]

use super::{
    parse::{
        consume_padding, try_consume_digits, try_consume_digits_in_range, try_consume_exact_digits,
        try_consume_exact_digits_in_range, try_consume_first_match,
    },
    Padding, ParseError, ParseResult, ParsedItems,
};
#[cfg(not(feature = "std"))]
use crate::no_std_prelude::*;
use crate::{Date, Language, Sign, Weekday};
use core::{
    fmt::{self, Formatter},
    num::{NonZeroU16, NonZeroU8},
};

/// Array of weekdays that corresponds to the localized values. This can be
/// zipped via an iterator to perform parsing easily.
const WEEKDAYS: [Weekday; 7] = [
    Weekday::Monday,
    Weekday::Tuesday,
    Weekday::Wednesday,
    Weekday::Thursday,
    Weekday::Friday,
    Weekday::Saturday,
    Weekday::Sunday,
];

/// Short day of the week
#[inline(always)]
pub(crate) fn fmt_a(f: &mut Formatter<'_>, date: Date, language: Language) -> fmt::Result {
    f.write_str(language.short_week_days()[date.weekday().number_days_from_monday() as usize])
}

/// Short day of the week
#[inline(always)]
pub(crate) fn parse_a(
    items: &mut ParsedItems,
    s: &mut &str,
    language: Language,
) -> ParseResult<()> {
    items.weekday = try_consume_first_match(
        s,
        language
            .short_week_days()
            .iter()
            .cloned()
            .zip(WEEKDAYS.iter().cloned()),
    )
    .ok_or(ParseError::InvalidDayOfWeek)?
    .into();

    Ok(())
}

/// Day of the week
#[inline(always)]
pub(crate) fn fmt_A(f: &mut Formatter<'_>, date: Date, language: Language) -> fmt::Result {
    f.write_str(language.week_days()[date.weekday().number_days_from_monday() as usize])
}

/// Day of the week
#[inline(always)]
pub(crate) fn parse_A(
    items: &mut ParsedItems,
    s: &mut &str,
    language: Language,
) -> ParseResult<()> {
    items.weekday = try_consume_first_match(
        s,
        language
            .week_days()
            .iter()
            .cloned()
            .zip(WEEKDAYS.iter().cloned()),
    )
    .ok_or(ParseError::InvalidDayOfWeek)?
    .into();

    Ok(())
}

/// Short month name
///
/// References on localization:
/// - [Yale](https://web.library.yale.edu/cataloging/months)
/// - [Princeton](https://library.princeton.edu/departments/tsd/katmandu/reference/months.html)
#[inline(always)]
pub(crate) fn fmt_b(f: &mut Formatter<'_>, date: Date, language: Language) -> fmt::Result {
    f.write_str(language.short_month_names()[date.month() as usize - 1])
}

/// Short month name
#[inline(always)]
pub(crate) fn parse_b(
    items: &mut ParsedItems,
    s: &mut &str,
    language: Language,
) -> ParseResult<()> {
    items.month = try_consume_first_match(s, language.short_month_names().iter().cloned().zip(1..))
        .map(NonZeroU8::new)
        .ok_or(ParseError::InvalidMonth)?;

    Ok(())
}

/// Month name
#[inline(always)]
pub(crate) fn fmt_B(f: &mut Formatter<'_>, date: Date, language: Language) -> fmt::Result {
    f.write_str(language.month_names()[date.month() as usize - 1])
}

/// Month name
#[inline(always)]
pub(crate) fn parse_B(
    items: &mut ParsedItems,
    s: &mut &str,
    language: Language,
) -> ParseResult<()> {
    items.month = try_consume_first_match(s, language.month_names().iter().cloned().zip(1..))
        .map(NonZeroU8::new)
        .ok_or(ParseError::InvalidMonth)?;

    Ok(())
}

/// Year divided by 100 and truncated to integer (`00`-`999`)
#[inline(always)]
pub(crate) fn fmt_C(f: &mut Formatter<'_>, date: Date, padding: Padding) -> fmt::Result {
    pad!(f, padding(Zero), 2, date.year() / 100)
}

/// Year divided by 100 and truncated to integer (`00`-`999`)
#[inline(always)]
pub(crate) fn parse_C(items: &mut ParsedItems, s: &mut &str, padding: Padding) -> ParseResult<()> {
    let padding_length = consume_padding(s, padding.default_to(Padding::Zero), 1);
    items.year = (try_consume_digits::<i32, _>(s, (2 - padding_length)..=(3 - padding_length))
        .ok_or(ParseError::InvalidYear)?
        * 100
        + items.year.unwrap_or(0).rem_euclid(100))
    .into();

    Ok(())
}

/// Day of the month, zero-padded (`01`-`31`)
#[inline(always)]
pub(crate) fn fmt_d(f: &mut Formatter<'_>, date: Date, padding: Padding) -> fmt::Result {
    pad!(f, padding(Zero), 2, date.day())
}

/// Day of the month, zero-padded (`01`-`31`)
#[inline(always)]
pub(crate) fn parse_d(items: &mut ParsedItems, s: &mut &str, padding: Padding) -> ParseResult<()> {
    items.day = try_consume_exact_digits::<u8>(s, 2, padding.default_to(Padding::Zero))
        .map(NonZeroU8::new)
        .ok_or(ParseError::InvalidDayOfMonth)?;

    Ok(())
}

/// Day of the month, space-padded (` 1`-`31`)
#[inline(always)]
pub(crate) fn fmt_e(f: &mut Formatter<'_>, date: Date, padding: Padding) -> fmt::Result {
    pad!(f, padding(Space), 2, date.day())
}

/// Day of the month, space-padded (` 1`-`31`)
#[inline(always)]
pub(crate) fn parse_e(items: &mut ParsedItems, s: &mut &str, padding: Padding) -> ParseResult<()> {
    parse_d(items, s, padding.default_to(Padding::Space))
}

/// Week-based year, last two digits (`00`-`99`)
#[inline(always)]
pub(crate) fn fmt_g(f: &mut Formatter<'_>, date: Date, padding: Padding) -> fmt::Result {
    pad!(f, padding(Zero), 2, date.iso_year_week().0.rem_euclid(100))
}

/// Week-based year, last two digits (`00`-`99`)
#[inline(always)]
pub(crate) fn parse_g(items: &mut ParsedItems, s: &mut &str, padding: Padding) -> ParseResult<()> {
    items.week_based_year = (items.week_based_year.unwrap_or(0) / 100 * 100
        + try_consume_exact_digits::<i32>(s, 2, padding.default_to(Padding::Zero))
            .ok_or(ParseError::InvalidYear)?)
    .into();

    Ok(())
}

/// Week-based year
#[inline(always)]
pub(crate) fn fmt_G(f: &mut Formatter<'_>, date: Date, padding: Padding) -> fmt::Result {
    pad!(f, padding(Zero), 4, date.iso_year_week().0)
}

/// Week-based year
#[inline(always)]
pub(crate) fn parse_G(items: &mut ParsedItems, s: &mut &str, padding: Padding) -> ParseResult<()> {
    let sign = try_consume_first_match(
        s,
        [("+", Sign::Positive), ("-", Sign::Negative)]
            .iter()
            .cloned(),
    )
    .unwrap_or(Sign::Positive);

    consume_padding(s, padding.default_to(Padding::Zero), 4);

    items.week_based_year = try_consume_digits_in_range(s, 1..=6, -100_000..=100_000)
        .map(|v: i32| sign * v)
        .ok_or(ParseError::InvalidYear)?
        .into();

    Ok(())
}

/// Day of the year, zero-padded to width 3 (`001`-`366`)
#[inline(always)]
pub(crate) fn fmt_j(f: &mut Formatter<'_>, date: Date, padding: Padding) -> fmt::Result {
    pad!(f, padding(Zero), 3, date.ordinal())
}

/// Day of the year, zero-padded to width 3 (`001`-`366`)
#[inline(always)]
pub(crate) fn parse_j(items: &mut ParsedItems, s: &mut &str, padding: Padding) -> ParseResult<()> {
    items.ordinal_day =
        try_consume_exact_digits::<NonZeroU16>(s, 3, padding.default_to(Padding::Zero))
            .ok_or(ParseError::InvalidDayOfYear)?
            .into();

    Ok(())
}

/// Month of the year, zero-padded (`01`-`12`)
#[inline(always)]
pub(crate) fn fmt_m(f: &mut Formatter<'_>, date: Date, padding: Padding) -> fmt::Result {
    pad!(f, padding(Zero), 2, date.month())
}

/// Month of the year, zero-padded (`01`-`12`)
#[inline(always)]
pub(crate) fn parse_m(items: &mut ParsedItems, s: &mut &str, padding: Padding) -> ParseResult<()> {
    items.month = try_consume_exact_digits::<NonZeroU8>(s, 2, padding.default_to(Padding::Zero))
        .ok_or(ParseError::InvalidMonth)?
        .into();

    Ok(())
}

/// ISO weekday (Monday = `1`, Sunday = `7`)
#[inline(always)]
pub(crate) fn fmt_u(f: &mut Formatter<'_>, date: Date) -> fmt::Result {
    write!(f, "{}", date.weekday().iso_weekday_number())
}

/// ISO weekday (Monday = `1`, Sunday = `7`)
#[inline(always)]
pub(crate) fn parse_u(items: &mut ParsedItems, s: &mut &str) -> ParseResult<()> {
    items.weekday = try_consume_first_match(
        s,
        (1..).map(|d| d.to_string()).zip(WEEKDAYS.iter().cloned()),
    )
    .ok_or(ParseError::InvalidDayOfWeek)?
    .into();

    Ok(())
}

/// Sunday-based week number (`00`-`53`)
#[inline(always)]
pub(crate) fn fmt_U(f: &mut Formatter<'_>, date: Date, padding: Padding) -> fmt::Result {
    pad!(f, padding(Zero), 2, date.sunday_based_week())
}

/// Sunday-based week number (`00`-`53`)
#[inline(always)]
pub(crate) fn parse_U(items: &mut ParsedItems, s: &mut &str, padding: Padding) -> ParseResult<()> {
    items.sunday_week =
        try_consume_exact_digits_in_range(s, 2, 0..=53, padding.default_to(Padding::Zero))
            .ok_or(ParseError::InvalidWeek)?
            .into();

    Ok(())
}

/// ISO week number, zero-padded (`01`-`53`)
#[inline(always)]
pub(crate) fn fmt_V(f: &mut Formatter<'_>, date: Date, padding: Padding) -> fmt::Result {
    pad!(f, padding(Zero), 2, date.week())
}

/// ISO week number, zero-padded (`01`-`53`)
#[inline(always)]
pub(crate) fn parse_V(items: &mut ParsedItems, s: &mut &str, padding: Padding) -> ParseResult<()> {
    items.iso_week =
        try_consume_exact_digits_in_range(s, 2, 1..=53, padding.default_to(Padding::Zero))
            .map(NonZeroU8::new)
            .ok_or(ParseError::InvalidWeek)?;

    Ok(())
}

/// Weekday number (Sunday = `0`, Saturday = `6`)
#[inline(always)]
pub(crate) fn fmt_w(f: &mut Formatter<'_>, date: Date) -> fmt::Result {
    write!(f, "{}", date.weekday().number_days_from_sunday())
}

/// Weekday number (Sunday = `0`, Saturday = `6`)
#[inline(always)]
pub(crate) fn parse_w(items: &mut ParsedItems, s: &mut &str) -> ParseResult<()> {
    let mut weekdays = WEEKDAYS;
    weekdays.rotate_left(1);

    items.weekday = try_consume_first_match(
        s,
        (0..)
            .map(|d: u8| d.to_string())
            .zip(weekdays.iter().cloned()),
    )
    .ok_or(ParseError::InvalidDayOfWeek)?
    .into();

    Ok(())
}

/// Monday-based week number (`00`-`53`)
#[inline(always)]
pub(crate) fn fmt_W(f: &mut Formatter<'_>, date: Date, padding: Padding) -> fmt::Result {
    pad!(f, padding(Zero), 2, date.monday_based_week())
}

/// Monday-based week number (`00`-`53`)
#[inline(always)]
pub(crate) fn parse_W(items: &mut ParsedItems, s: &mut &str, padding: Padding) -> ParseResult<()> {
    items.monday_week =
        try_consume_exact_digits_in_range(s, 2, 0..=53, padding.default_to(Padding::Zero))
            .ok_or(ParseError::InvalidWeek)?
            .into();

    Ok(())
}

/// Last two digits of year (`00`-`99`)
#[inline(always)]
pub(crate) fn fmt_y(f: &mut Formatter<'_>, date: Date, padding: Padding) -> fmt::Result {
    pad!(f, padding(Zero), 2, date.year().rem_euclid(100))
}

/// Last two digits of year (`00`-`99`)
#[inline(always)]
pub(crate) fn parse_y(items: &mut ParsedItems, s: &mut &str, padding: Padding) -> ParseResult<()> {
    items.year = (items.year.unwrap_or(0) / 100 * 100
        + try_consume_exact_digits::<i32>(s, 2, padding.default_to(Padding::Zero))
            .ok_or(ParseError::InvalidYear)?)
    .into();

    Ok(())
}

/// Full year
#[inline(always)]
pub(crate) fn fmt_Y(f: &mut Formatter<'_>, date: Date, padding: Padding) -> fmt::Result {
    let year = date.year();

    if year >= 10_000 {
        f.write_str("+")?;
    }

    pad!(f, padding(Zero), 4, year)
}

/// Full year
#[inline(always)]
pub(crate) fn parse_Y(items: &mut ParsedItems, s: &mut &str, padding: Padding) -> ParseResult<()> {
    let sign = try_consume_first_match(
        s,
        [("+", Sign::Positive), ("-", Sign::Negative)]
            .iter()
            .cloned(),
    )
    .unwrap_or(Sign::Positive);

    consume_padding(s, padding.default_to(Padding::Zero), 4);

    items.year = try_consume_digits_in_range(s, 1..=6, -100_000..=100_000)
        .map(|v: i32| sign * v)
        .ok_or(ParseError::InvalidYear)?
        .into();

    Ok(())
}
