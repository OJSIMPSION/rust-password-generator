use std::io;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
fn generate(){
    println!("How long? : ");
    let mut length = String::new();
    io::stdin().read_line(&mut length).expect("failed to read line");
    let length_int = length.trim().parse().expect("not numb, or numb is beyond i128 integer limit");
    let password : String = thread_rng().sample_iter(&Alphanumeric).take(length_int).map(char::from).collect();
    println!("{}", password)
}

fn main() {
    let mut would_like = String::new();
    println!("Would you like a password? : ");
    io::stdin().read_line(&mut would_like).expect("failed to read line");
    let would_like_str = would_like.trim();
    match would_like_str {
        "Yes" | "yes" => generate(),
        _ => println!("No password generated")
    }

}
