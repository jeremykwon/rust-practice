use rand; // rand 라이브러리를 가져옴

use std::io; // std 표준 라이브러리에서 io 를 가져옴, 요렇게 하지않는다면 std::io::stdin() 이런식으로 사용해야함
use rand::Rng; // rand 라이브러리에서 Rng 트레이트를 가져옴

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);
    
    let mut guess = String::new(); // 빈 문자열을 만들어서 guess 에 할당

    io::stdin().read_line(&mut guess) // io의 연관함수인 stdin을 호출
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}