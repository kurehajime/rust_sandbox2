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
    let top = word.chars().nth(0).unwrap();
    if aiueo.contains(&top) {
        word.to_owned() + "hay"
    } else {
        slice(word, 1..word.char_indices().count()) + &top.to_string() + "ay"
    }
}

fn slice(s: &str, range: Range<usize>) -> String {
    let mut result = String::new();
    let mut index = 0;
    for (_, c) in s.char_indices() {
        if index >= range.start && index < range.end {
            result.push_str(&c.to_string());
        }
        index = index + 1;
    }
    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn pig() {
        assert_eq!(super::pig("hello"), "ellohay");
        assert_eq!(super::pig("apple"), "applehay");
        assert_eq!(super::pig("はろー"), "ろーはay");
    }
}
