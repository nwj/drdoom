use anyhow::Result;
use chrono::prelude::*;
use chrono::{Duration, Weekday};
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use rustyline::Editor;
use std::fmt;

fn main() -> Result<()> {
    let mut rng = ChaCha20Rng::from_entropy();
    let mut rl = Editor::<()>::new();
    let mut stats = Stats::default();
    let mut random_date;
    let mut prompt_time;
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
            input = rl.readline(">> ")?;

            match parse_command(input) {
                Some(Command::Quit) => {
                    println!("Quitting...");
                    return Ok(());
                }
                Some(Command::Guess(weekday)) => {
                    input_weekday = weekday;
                    break;
                }
                None => {
                    println!("Unrecognized input. Try again.");
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

enum Command {
    Quit,
    Guess(Weekday),
}

fn generate_date(mut rng: impl rand::Rng) -> NaiveDate {
    let random = rng.gen_range(-DAYS_IN_TWO_CENTURIES..DAYS_IN_TWO_CENTURIES);
    let now = Local::now().date().naive_local();

    match random {
        0 => now,
        // Overflow is theoretically possible here, but unlikely, so I'm not using checked_add_signed.
        _ => now + Duration::days(random),
    }
}

fn parse_command(input: String) -> Option<Command> {
    match input.to_lowercase().trim() {
        "q" | "quit" => Some(Command::Quit),
        "m" | "mo" | "mon" | "monday" => Some(Command::Guess(Weekday::Mon)),
        "tu" | "tue" | "tues" | "tuesday" => Some(Command::Guess(Weekday::Tue)),
        "w" | "we" | "wed" | "wednesday" => Some(Command::Guess(Weekday::Wed)),
        "th" | "thu" | "thur" | "thurs" | "thursday" => Some(Command::Guess(Weekday::Thu)),
        "f" | "fr" | "fri" | "friday" => Some(Command::Guess(Weekday::Fri)),
        "sa" | "sat" | "saturday" => Some(Command::Guess(Weekday::Sat)),
        "su" | "sun" | "sunday" => Some(Command::Guess(Weekday::Sun)),
        _ => None,
    }
}

fn display_weekday(weekday: chrono::Weekday) -> &'static str {
    match weekday {
        Weekday::Mon => "Monday",
        Weekday::Tue => "Tuesday",
        Weekday::Wed => "Wednesday",
        Weekday::Thu => "Thursday",
        Weekday::Fri => "Friday",
        Weekday::Sat => "Saturday",
        Weekday::Sun => "Sunday",
    }
}

fn is_was(date: NaiveDate) -> &'static str {
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
    last_duration: Duration,
    avg_duration: Duration,
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
            last_duration: Duration::seconds(0),
            avg_duration: Duration::seconds(0),
        }
    }
}
