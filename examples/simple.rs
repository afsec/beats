#![warn(clippy::all)]
// source: https://en.wikipedia.org/wiki/Swatch_Internet_Time

use beats::get_current_beats;

fn main() {
    println!("@{:03}", get_current_beats());
}
