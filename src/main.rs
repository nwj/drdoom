use anyhow::{anyhow, Result};
use chrono::prelude::*;
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use rustyline::Editor;

// Note that this value can be off by 1 depending on what centuries we're talking about.
// Centuries that are cleanly divisible by 400 (e.g. 1600, 2000, etc.) have an extra "century leap year".
// Centuries _with_ a century leap year have 36,525 days.
// Centuries _without_ a century leap year have 36,524 days.
const DAYS_IN_TWO_CENTURIES: i64 = 73048;

fn main() {
    let mut rng = ChaCha20Rng::from_entropy();
    let mut rl = Editor::<()>::new();

    let mut max_streak = 0;
    let mut current_streak = 0;
    let mut correct_guesses = 0;
    let mut total_guesses = 0;
    let mut average_duration = chrono::Duration::seconds(0);
    let mut start_time;
    let mut stop_time;
    let mut random_date;
    let mut duration;

    let mut input;
    let mut input_weekday;

    loop {
        random_date = generate_date(&mut rng);
        start_time = Local::now();
        println!(
            "What day of the week {} {}?",
            is_was(random_date),
            random_date.format("%B %d, %Y")
        );

        loop {
            input = rl.readline(">> ").unwrap();
            match parse_weekday(input) {
                Ok(weekday) => {
                    input_weekday = weekday;
                    break;
                }
                Err(_) => {
                    println!("Unrecognized weekday. Try again.");
                }
            }
        }
        total_guesses += 1;
        stop_time = Local::now();
        duration = stop_time - start_time;
        average_duration = average_duration + ((duration - average_duration) / total_guesses);

        if input_weekday == random_date.weekday() {
            println!(
                "Yes! {} {} a {}.",
                random_date.format("%B %d, %Y"),
                is_was(random_date),
                display_weekday(random_date.weekday())
            );

            correct_guesses += 1;
            current_streak += 1;
            if current_streak > max_streak {
                max_streak = current_streak;
            }
        } else {
            println!(
                "Nope. {} {} a {}.",
                random_date.format("%B %d, %Y"),
                is_was(random_date),
                display_weekday(random_date.weekday())
            );

            current_streak = 0;
        }

        println!(
            "Correct: {} / {} ({:.1}%) | Streak: {} | Duration: {:.2}s\nBest Streak: {} | Average Duration: {:.2}s\n",
            correct_guesses,
            total_guesses,
            (correct_guesses as f64 / total_guesses as f64) * 100.0,
            current_streak,
            duration.num_milliseconds() as f64 / 1000.0,
            max_streak,
            average_duration.num_milliseconds() as f64 / 1000.0,
        );
    }
}

fn generate_date(mut rng: impl rand::Rng) -> chrono::NaiveDate {
    let random = rng.gen_range(-DAYS_IN_TWO_CENTURIES..DAYS_IN_TWO_CENTURIES);
    let now = Local::now().date().naive_local();

    match random {
        0 => now,
        // Overflow is theoretically possible here, but unlikely, so I'm not using checked_add_signed.
        _ => now + chrono::Duration::days(random),
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

fn is_was(date: chrono::NaiveDate) -> &'static str {
    if date >= Local::now().date().naive_local() {
        "is"
    } else {
        "was"
    }
}
