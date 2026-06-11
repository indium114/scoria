use colored::*;
use std::fmt;

// MARK: reusable print function
fn generic_print(prefix: ColoredString, text: impl fmt::Display) {
    println!("{prefix} {text}")
}
fn generic_err(prefix: ColoredString, text: impl fmt::Display) {
    eprintln!("{prefix} {text}")
}

// MARK: actual log thingies.
pub fn ok(text: impl fmt::Display) {
    generic_print("[ok]".green(), text);
}

pub fn warn(text: impl fmt::Display) {
    generic_print("[warn]".yellow(), text);
}

pub fn err(text: impl fmt::Display) {
    generic_err("[err]".red(), text);
}

pub fn sync(text: impl fmt::Display) {
    generic_print("sync".blue(), text)
}

pub fn update(text: impl fmt::Display) {
    generic_print("[update]".magenta(), text);
}

pub fn log(text: impl fmt::Display) {
    generic_print("[log]".black().bold(), text);
}

pub fn clean(text: impl fmt::Display) {
    generic_print("[clean]".cyan(), text);
}

pub fn add(text: impl fmt::Display) {
    generic_print("[add]".magenta().bold(), text);
}

pub fn hint(text: impl fmt::Display) {
    generic_print("[hint]".cyan().bold(), text);
}

pub fn query(text: impl fmt::Display) {
    generic_print("[query]".yellow().bold(), text);
}
