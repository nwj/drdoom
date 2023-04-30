use anyhow::Result;
use chrono::prelude::*;
use chrono::{Duration, Weekday};
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
        prompt_time = Local::now();
        println!(
            "What day of the week {}: {}?",
            is_was(random_date),
            random_date.format("%B %d, %Y")
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
                            random_date.format("%B %d, %Y"),
                            is_was(random_date),
                            display_weekday(random_date.weekday())
                        );
                    } else {
                        stats.increment_incorrect(prompt_time);
                        println!(
                            "❌ Nope. {} {} a {}.",
                            random_date.format("%B %d, %Y"),
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

fn generate_date(mut rng: impl rand::Rng) -> NaiveDate {
    // October 15, 1582 is explicitly selected as the default lower end of the
    // range from which we will select random dates because it is the earliest
    // date on which the Gregorian calendar was adopted.
    let default_start_date = NaiveDate::from_ymd_opt(1582, 10, 15).unwrap();

    let now = Local::now().date_naive();
    if now < default_start_date {
        panic!("Current date preceeds the adoption of the Gregorian calendar...)")
    }

    let start_to_now = now - default_start_date;
    let now_to_end = Duration::days(365 * 50);

    let random = rng.gen_range(-start_to_now.num_days()..now_to_end.num_days());

    match random {
        0 => now,
        _ => now + Duration::days(random),
    }
}

fn parse_command(input: String) -> Option<Command> {
    match input.to_lowercase().trim() {
        "q" | "quit" | "exit" => Some(Command::Quit),
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

fn display_weekday(weekday: Weekday) -> &'static str {
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
    if date >= Local::now().date_naive() {
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
        write!(f, "\nStreak: {} | Duration: {:.2}s\nCorrect: {} / {} ({:.1}%) | Best Streak: {} | Avg. Duration: {:.2}s\n",
            self.current_streak,
            self.last_duration.num_milliseconds() as f64 / 1000.0,
            self.correct_guesses,
            self.total_guesses,
            (self.correct_guesses as f64 / self.total_guesses as f64) * 100.0,
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
