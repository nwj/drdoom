use anyhow::{anyhow, Result};
use chrono::prelude::*;
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use rustyline::Editor;
use std::fmt;

fn main() {
    let mut rng = ChaCha20Rng::from_entropy();
    let mut rl = Editor::<()>::new();
    let mut random_date;

    let mut prompt_time;
    let mut stats = Stats::default();

    let mut input;
    let mut input_weekday;

    loop {
        random_date = generate_date(&mut rng);
        prompt_time = Local::now();
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

        if input_weekday == random_date.weekday() {
            stats.increment_correct(prompt_time);
            println!(
                "Yes! {} {} a {}.",
                random_date.format("%B %d, %Y"),
                is_was(random_date),
                display_weekday(random_date.weekday())
            );
        } else {
            stats.increment_incorrect(prompt_time);
            println!(
                "Nope. {} {} a {}.",
                random_date.format("%B %d, %Y"),
                is_was(random_date),
                display_weekday(random_date.weekday())
            );
        }

        println!("{}", stats);
    }
}

// Note that this value can be off by 1 depending on what centuries we're talking about.
// Centuries that are cleanly divisible by 400 (e.g. 1600, 2000, etc.) have an extra "century leap year".
// Centuries _with_ a century leap year have 36,525 days.
// Centuries _without_ a century leap year have 36,524 days.
const DAYS_IN_TWO_CENTURIES: i64 = 73048;

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

#[derive(Debug)]
struct Stats {
    total_guesses: u32,
    correct_guesses: u32,
    current_streak: u32,
    best_streak: u32,
    last_duration: chrono::Duration,
    avg_duration: chrono::Duration,
}

impl Stats {
    fn increment_correct(&mut self, prompt_time: DateTime<Local>) -> () {
        self.total_guesses += 1;
        self.correct_guesses += 1;
        self.current_streak += 1;
        if self.current_streak > self.best_streak {
            self.best_streak = self.current_streak
        }
        self.update_durations(prompt_time);
    }

    fn increment_incorrect(&mut self, prompt_time: DateTime<Local>) -> () {
        self.total_guesses += 1;
        self.current_streak = 0;
        self.update_durations(prompt_time);
    }

    fn update_durations(&mut self, prompt_time: DateTime<Local>) -> () {
        self.last_duration = Local::now() - prompt_time;
        self.avg_duration = self.avg_duration
            + ((self.last_duration - self.avg_duration) / self.total_guesses as i32);
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Correct: {} / {} ({:.1}%) | Streak: {} | Duration: {:.2}s\nBest Streak: {} | Avg. Duration: {:.2}s\n",
            self.correct_guesses,
            self.total_guesses,
            (self.correct_guesses as f64 / self.total_guesses as f64) * 100.0,
            self.current_streak,
            self.last_duration.num_milliseconds() as f64 / 1000.0,
            self.best_streak,
            self.avg_duration.num_milliseconds() as f64 / 1000.0,
        )
    }
}

impl Default for Stats {
    fn default() -> Self {
        Stats {
            total_guesses: 0,
            correct_guesses: 0,
            current_streak: 0,
            best_streak: 0,
            last_duration: chrono::Duration::seconds(0),
            avg_duration: chrono::Duration::seconds(0),
        }
    }
}
