use anyhow::Result;
use jiff::civil::{date, Date, Weekday};
use jiff::{Span, SpanRound, Timestamp, Unit, Zoned};
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use rustyline::DefaultEditor;
use std::fmt;

fn main() -> Result<()> {
    let mut rng = ChaCha20Rng::from_entropy();
    let mut rl = DefaultEditor::new()?;
    let mut stats = Stats::default();
    let mut random_date;
    let mut prompt_time;

    loop {
        random_date = generate_date(&mut rng);
        prompt_time = Timestamp::now();
        println!(
            "What day of the week {}: {}?",
            is_was(random_date),
            random_date.strftime("%B %d, %Y")
        );

        loop {
            match parse_command(rl.readline(">> ")?) {
                None => {
                    println!("Unrecognized input. Try again.");
                }
                Some(Command::Guess(weekday)) => {
                    if weekday == random_date.weekday() {
                        stats.increment_correct(prompt_time);
                        println!(
                            "✅ Yes! {} {} a {}.",
                            random_date.strftime("%B %d, %Y"),
                            is_was(random_date),
                            display_weekday(random_date.weekday())
                        );
                    } else {
                        stats.increment_incorrect(prompt_time);
                        println!(
                            "❌ Nope. {} {} a {}.",
                            random_date.strftime("%B %d, %Y"),
                            is_was(random_date),
                            display_weekday(random_date.weekday())
                        );
                    }
                    println!("{}", stats);

                    println!("Press [ENTER] to continue");
                    match rl.readline("")? {
                        _ => {
                            break;
                        }
                    }
                }
                Some(Command::Quit) => {
                    println!("Quitting...");
                    return Ok(());
                }
            }
        }
    }
}

enum Command {
    Guess(Weekday),
    Quit,
}

fn generate_date(mut rng: impl rand::Rng) -> Date {
    // October 15, 1582 is explicitly selected as the default lower end of the
    // range from which we will select random dates because it is the earliest
    // date on which the Gregorian calendar was adopted.
    let default_start_date: Date = date(1582, 10, 15);

    let now = Zoned::now();
    let now_date: Date = now.into();
    if now_date < default_start_date {
        panic!("Current system date preceeds the adoption of the Gregorian calendar...)")
    }

    let start_to_now = default_start_date.until(now_date).unwrap();
    let now_to_end = Span::new()
        .years(100)
        .round(SpanRound::new().largest(Unit::Day).relative(now_date))
        .unwrap();

    let random = rng.gen_range(-start_to_now.get_days()..now_to_end.get_days());

    match random {
        0 => now_date,
        _ => now_date + Span::new().days(random),
    }
}

fn parse_command(input: String) -> Option<Command> {
    match input.to_lowercase().trim() {
        "q" | "quit" | "exit" => Some(Command::Quit),
        "m" | "mo" | "mon" | "monday" => Some(Command::Guess(Weekday::Monday)),
        "tu" | "tue" | "tues" | "tuesday" => Some(Command::Guess(Weekday::Tuesday)),
        "w" | "we" | "wed" | "wednesday" => Some(Command::Guess(Weekday::Wednesday)),
        "th" | "thu" | "thur" | "thurs" | "thursday" => Some(Command::Guess(Weekday::Thursday)),
        "f" | "fr" | "fri" | "friday" => Some(Command::Guess(Weekday::Friday)),
        "sa" | "sat" | "saturday" => Some(Command::Guess(Weekday::Saturday)),
        "su" | "sun" | "sunday" => Some(Command::Guess(Weekday::Sunday)),
        _ => None,
    }
}

fn display_weekday(weekday: Weekday) -> &'static str {
    match weekday {
        Weekday::Monday => "Monday",
        Weekday::Tuesday => "Tuesday",
        Weekday::Wednesday => "Wednesday",
        Weekday::Thursday => "Thursday",
        Weekday::Friday => "Friday",
        Weekday::Saturday => "Saturday",
        Weekday::Sunday => "Sunday",
    }
}

fn is_was(date: Date) -> &'static str {
    if date >= Zoned::now().into() {
        "is"
    } else {
        "was"
    }
}

#[derive(Debug, Default)]
struct Stats {
    total_guesses: u32,
    correct_guesses: u32,
    current_streak: u32,
    best_streak: u32,
    last_duration: f64,
    avg_duration: f64,
}

impl Stats {
    fn increment_correct(&mut self, prompt_time: Timestamp) -> () {
        self.total_guesses += 1;
        self.correct_guesses += 1;
        self.current_streak += 1;
        if self.current_streak > self.best_streak {
            self.best_streak = self.current_streak
        }
        self.update_durations(prompt_time);
    }

    fn increment_incorrect(&mut self, prompt_time: Timestamp) -> () {
        self.total_guesses += 1;
        self.current_streak = 0;
        self.update_durations(prompt_time);
    }

    fn update_durations(&mut self, prompt_time: Timestamp) -> () {
        self.last_duration = (Timestamp::now() - prompt_time)
            .total(Unit::Second)
            .unwrap();
        self.avg_duration = self.avg_duration
            + ((self.last_duration - self.avg_duration) / self.total_guesses as f64);
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\nStreak: {} | Duration: {:.2}s\nCorrect: {} / {} ({:.1}%) | Best Streak: {} | Avg. Duration: {:.2}s\n",
            self.current_streak,
            self.last_duration,
            self.correct_guesses,
            self.total_guesses,
            (self.correct_guesses as f64 / self.total_guesses as f64) * 100.0,
            self.best_streak,
            self.avg_duration,
        )
    }
}
