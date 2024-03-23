use std::{cmp::Ordering, io};
use rand::Rng; 

fn main() {
    println!("숫자를 추측하세요!");

    let secret_num = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("숫자를 입력하세요.");
        
        let mut guess = String::new(); 

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess:u32  = match guess.trim().parse() {
            Ok(num)=> num,
            Err(_)=>continue
        };

        println!("당신의 추측: {guess}");
    
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("너무 작어요."),
            Ordering::Greater => println!("너무 커요."),
            Ordering::Equal => {
                println!("정답.");
                break;
            }
        }
    }

} 