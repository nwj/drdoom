use anyhow::{anyhow, Result};
use chrono::prelude::*;
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use std::io;

// Note that this value can be off by 1 depending on what centuries we're talking about.
// Centuries that are cleanly divisible by 400 (e.g. 1600, 2000, etc.) have an extra "century leap year".
// Centuries _with_ a century leap year have 36,525 days.
// Centuries _without_ a century leap year have 36,524 days.
const DAYS_IN_TWO_CENTURIES: i64 = 73048;

fn main() {
    let mut rng = ChaCha20Rng::from_entropy();
    let random = rng.gen_range(-DAYS_IN_TWO_CENTURIES..DAYS_IN_TWO_CENTURIES);
    let now = Local::now().date().naive_local();

    let random_date = match random {
        0 => now,
        _ => now
            .checked_add_signed(chrono::Duration::days(random))
            .unwrap(),
    };

    println!("What day of the week was: {}?", random_date);

    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer).unwrap();
    match parse_weekday(buffer) {
        Ok(d) => println!("{}, got it.", display_weekday(d)),
        Err(_) => println!("Unrecognized weekday."),
    }
}

fn parse_weekday(input: String) -> Result<chrono::Weekday> {
    match input.to_lowercase().trim() {
        "m" | "mon" | "monday" => Ok(chrono::Weekday::Mon),
        "t" | "tu" | "tue" | "tues" | "tuesday" => Ok(chrono::Weekday::Tue),
        "w" | "wed" | "wednesday" => Ok(chrono::Weekday::Wed),
        "r" | "h" | "th" | "thu" | "thur" | "thurs" | "thursday" => Ok(chrono::Weekday::Thu),
        "f" | "fri" | "friday" => Ok(chrono::Weekday::Fri),
        "s" | "sa" | "sat" | "saturday" => Ok(chrono::Weekday::Sat),
        "u" | "su" | "sun" | "sunday" => Ok(chrono::Weekday::Sun),
        _ => Err(anyhow!("Could not determine weekday from input")),
    }
}

fn display_weekday(weekday: chrono::Weekday) -> &'static str {
    match weekday {
        chrono::Weekday::Mon => "Monday",
        chrono::Weekday::Tue => "Tuesday",
        chrono::Weekday::Wed => "Wednesday",
        chrono::Weekday::Thu => "Thursday",
        chrono::Weekday::Fri => "Friday",
        chrono::Weekday::Sat => "Saturday",
        chrono::Weekday::Sun => "Sunday",
    }
}
