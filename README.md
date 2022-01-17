# drdoom

A script for learning and practicing the Doomsday algorithm

## What is the Doomsday algorithm?

The [Doomsday algorithm](https://en.wikipedia.org/wiki/Doomsday_rule) is a method for determining the day of the week for a given date. The algorithm is simple enough that it can be performed quickly through mental calculation. For instance, John Conway, who devised the algorithm, was usually able to answer correctly within a few seconds.

## How does `drdoom` work?

The script repeatedly prompts you with randomly generated dates and asks you to identify the corresponding weekday. It also tracks statistics about your performance, such as how long on average you take to respond and how many consecutive dates you have correctly calculated.

## Installation

### From Source Code

`drdoom` is written in Rust, so you'll need to [install that](https://www.rust-lang.org/tools/install) in order to compile it.

To build:

```
$ git clone git@github.com:nwj/drdoom.git
$ cd drdoom
$ cargo build --release
$ ./target/release/drdoom
```

`drdoom` currently builds in a matter of seconds, so if you prefer, you can also just quickly run it with `cargo run`.
