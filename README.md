# RUST 스터디를 위한 디렉토리

## 실행

---

rustc 파일명.rs // 컴파일

./파일명 // 실행

## 패키지 매니저 cargo

- cargo란 rust 의 패키지 매니저 이다.

- <span style="background-color: #fff5b1; color: black;">cargo new hello_cargo --bin</span> // hello_cargo 실행가능한 바이너리 생성, bin 인자를 통해서 라이브러리가 아닌 실행 가능합 앱으로 만들어줌
  // hello world 가 출려고디는 main 함수가 만들어져있음

### cargo 프로젝트 빌드

- <span style="background-color: #fff5b1; color: black;">cargo build</span>
- ./target/debug/hello_cargo 혹은 <span style="background-color: #fff5b1; color: black;">cargo run</span> // 빌드된 프로젝트 실행
- cargo check // 코드가 컴파일되는지를 빠르게 확인해주지만 실행파일을 생성하지는 않는다.
- cargo build --release // 배포를 위해 최적화와 함께 빌드
- cargo update // Cargo.toml 에 명시한 요구사항에 맞는 최신 버전을 확인. 버전에 문제가 없다면 Cargo는 해당 버전을 Cargo.lock 에 기록한다. -> 0.3.0를 dependency에 적어두었다면 0.4.0 미만의 0.3.x 최신 버전을 다운받는다.
- cargo doc --open // 프로젝트 구조, 함수등의 정보가 doc 형태로 열림

## 추리게임 (input, output)

```rust

use std::io;

fn main() {
println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

}

```

- let 으로 변수를 선언한다
- rust에서 변수는 기본적으로 immutable(불변) 하다, mut 구문을 추가해 mutable 하게 설정해줘야 변경 가능

  ```
  let maple = "";
  let mut maple2 = "";
  ```

- :: 은 연관함수임을 나타낸다. 예를들어 io:stdin()은 io의 연관함수인 stdin 을 실행하는 구문이다.
- stdin의 read_line은 우리가 인자로 넘긴 문자열에 사용자가 입력을 저장할 뿐 아니라 하나의 값을 돌려준다.
  돌려준 값은 io::Result 이다.
  Result 타입은 열거형(enums, 정해진 값만 가짐)이다.
  열거형의 variants 라고 한다.

Result 타입의 variants는 Ok 와 Err이다.

io::Result 인스턴스는 expect 메소드를 가지고 있습니다. 만약 io::Result 인스턴스가 Err일 경우 expect 메소드는 프로그램이 작동을 멈추게 하고 expect에 인자로 넘겼던 메세지를 출력하도록 합니다

## 크레이트 (crate) == package

- package(코드묶름)를 rust에서는 크레이트(crate) 라고 한다.
- 컴파일 해서 실행 가능한 것은 binary crate이고 rand와 같은 다른 프로그램에서 사용되기 위한 라이브러리는 library crate 입니다.
