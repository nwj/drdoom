use chrono::prelude::*;
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

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

    println!("{}", random_date);
    println!("{}", random_date.format("%A"));
}
