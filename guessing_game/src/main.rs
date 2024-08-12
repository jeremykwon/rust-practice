use rand; // rand 라이브러리를 가져옴

use std::io; // std 표준 라이브러리에서 io 를 가져옴, 요렇게 하지않는다면 std::io::stdin() 이런식으로 사용해야함
use std::cmp::Ordering;
use rand::Rng; // rand 라이브러리에서 Rng 트레이트를 가져옴

fn main() {
    println!("비밀 숫자 맞추는 프로그램입니다. 당신이 추측한 번호를 입력하세요.");

    // thread_rng 함수는 OS가 시드(seed)를 정하고 현재 스레드에서만 사용되는 특별한 정수생성기를 돌려준다.
    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut loop_count: i64 = 0;

    loop {
        loop_count += 1;
        // rust 변수는 기본적으로 불변, mut 을 사용해 가변변수로 만들 수 있음
        let mut guess = String::new(); // 빈 문자열을 만들어서 guess 에 할당

        // read_line은 용자로부터 입력을 받기 위해 사용
        io::stdin().read_line(&mut guess) // io의 연관함수인 stdin을 호출
            .expect("Failed to read line");

        // 에러 처리 코드, u32은 부호가 없는 32비트의 정수
        // let guess: u32 = guess.trim().parse()
        //     .expect("Please type a number!");

        // 잘못된 입력값 처리하는 로직
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                println!("당신의 추측: {}", guess);
                num // 숫자인 경우 num으로 반환
            },
            Err(_) => {
                println!("숫자를 입력해주세요.");
                continue; // 숫자가 아닌 경우 무시하고 다시 입력받음
            }, 
        };

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("너무 작습니다."),
            Ordering::Greater => println!("너무 큽니다."),
            Ordering::Equal   => {
                println!("정답입니다!");
                println!("시도 횟수: {}", loop_count);
                break; // 루프를 빠져나감
            }
        }
    }
}