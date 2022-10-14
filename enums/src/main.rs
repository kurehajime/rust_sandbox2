enum Bird {
    Chicken(String),
    Uguisu,
    Duck(String),
}
fn main() {
    let duck = Bird::Duck("quack quack".to_string());
    let chicken = Bird::Chicken("cock-a-doodle-doo".to_string());
    let uguisu = Bird::Uguisu;

    say(&duck);
    say(&chicken);
    say(&uguisu);
}

fn say(bird: &Bird) {
    match bird {
        Bird::Chicken(cry) => println!("say {}", cry),
        Bird::Duck(cry) => println!("say {}", cry),
        Bird::Uguisu => {}
    }
}
