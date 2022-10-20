use std::fmt::Display;

trait BigToString {
    fn big_to_string(&self) -> String;
}

impl<T: Display> BigToString for T {
    fn big_to_string(&self) -> String {
        format!("{}", self).to_ascii_uppercase()
    }
}

fn main() {
    let s = "hello world";
    println!("{}", s.big_to_string());
}
