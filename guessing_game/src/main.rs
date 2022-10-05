use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("数当てゲーム");
    let secret_number = rand::thread_rng().gen_range(1..101); // 1から100までの乱数を生成

    loop {
        let mut guess = String::new();
        println!("数を入れてね");
        io::stdin()
            .read_line(&mut guess)
            .expect("読み込みに失敗しました");

        println!("あなたの入力: {}", guess);
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("それは小さすぎる"),
            Ordering::Greater => println!("さすがに大き過ぎる"),
            Ordering::Equal => {
                println!("正解!");
                break;
            }
        }
    }
}
