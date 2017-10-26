extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("1~20 까지 숫자중 하나를 맞춰보세요!");

    let number = rand::thread_rng().gen_range(1,21);

    loop {
        println!("당신이 예상한 숫자를 입력하세요.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .ok()
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num)  => num,
            Err(_)   => continue,
        };

        match guess.cmp(&number) {
            Ordering::Less     => println!("{}보다 큽니다", guess),
            Ordering::Greater  => println!("{}보다 작아요", guess),
            Ordering::Equal    => {
                println!("정답입니다!");
                break;
            }
        }
    }
}