use colored::*;
use std::fmt;

fn generic_print(prefix: ColoredString, msg: impl fmt::Display) {
    println!("{prefix} {msg}")
}
