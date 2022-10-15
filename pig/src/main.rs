use std::io;
use std::ops::Range;

fn main() {
    let mut word = String::new();
    println!("英単語を入れてね");
    io::stdin()
        .read_line(&mut word)
        .expect("読み込みに失敗しました");
    let pig_word = pig(&word.trim());
    println!("{}", pig_word);
}

fn pig(word: &str) -> String {
    let aiueo = ['a', 'i', 'u', 'e', 'o'];
    let pig_word: String;
    let top = word.chars().nth(0).unwrap();
    if aiueo.contains(&top) {
        pig_word = word.to_owned() + "hay";
    } else {
        pig_word = slice(
            word,
            Range {
                start: 1,
                end: word.char_indices().count(),
            },
        ) + &top.to_string()
            + "ay";
    }
    pig_word
}

fn slice(s: &str, range: Range<usize>) -> String {
    let mut result = String::new();

    for (i, c) in s.char_indices() {
        if i >= range.start && i < range.end {
            result.push_str(&c.to_string());
        }
    }
    result
}
