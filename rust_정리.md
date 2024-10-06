## 조건문

```
match guess.cmp(&secret_number) {
	Ordering::Less    => println!("Too small!"),
	Ordering::Greater => println!("Too big!"),
	Ordering::Equal   => println!("You win!"),
}
```

cmp 메소드는 두 값을 비교하며 비교 가능한 모든 것들에 대해 호출할 수 있다.
(위 코드에서는 guess와 secret_number를 비교)

<span style="background-color: #fff5b1; color: black;">match 표현문을 이용하여 cmp가 guess와 secret_number를 비교한 결과인 Ordering의 값에 따라 무엇을 할 것인지 결정할 수 있다.</span>

## 변수의 형식

```
// u32은 부호가 없는 32비트의 정수
let guess: u32 = 10;

// i32는 32비트 정수
let guess: i32 = 10;

// i64는 64 비트 정수
let guess: i642 = 10;

// const 로 상수 정의
const MAX_POINTS: u32 = 100_000;

// mut 키워드를 통해 가변 변수 선언
let mut loop_count: i64 = 0; 
```

## shadows
```
// shadows - 변수를 재선언해 값을 덮어씌우는 것
fn main() {
    let x = 5;

    let x = x + 1; // 기존 x 는 5 이므로 해당 라인에서 x는 6이 된다.

    let x = x * 2; // 기존 x 는 6 이므로 헤당 라인에서 x는 12가 된다. 

    println!("The value of x is: {}", x);
}
```
mut 과 다른점이 있다면 변수가 불변하게 유지된다는 점이다
(let 키워드를 사용하지 않는 이상 변수 변경 불가능)
또한 <u><b>mut은 저장된 데이터의 타입을 변경시 에러가 나는데 shadows 시에는 다른 타입이 삽입되어도 x문제가 되지 않는다</b></u>

## 변수의 타입
정수, 부동 소수점 숫자, boolean, 문자열
위 네가지 타입을 rust에서는 스칼라 타입이라고 한다.

- 정수

| Length | Signed | Unsigned |
|--------|--------|----------|
| 8-bit  | i8     | u8       |
| 16-bit | i16    | u16      |
| 32-bit | i32    | u32      |
| 64-bit | i64    | u64      |
| arch   | isize  | usize    |

arch의 경우 본인의 컴퓨터 환경(아키텍처) 에 따라 결정된다.

일반적으로 i32가 가장 빠르기에 일반적인 경우에는 i32를 사용하면 된다. (64-bit 시스템에서도)

<br>

- 부동 소수점
	- f64, f32

<br>

- boolean
	- bool

<br>

- 튜플
	- 다양한 타입의 값들을 하나의 타입으로 묶을 수 있다.
	```
	fn main() {
    	let tup: (i32, f64, u8) = (500, 6.4, 1);
	}

	let (x, y, z) = tup; // 구조 해제

	// .index 를 통해 값에 접근가능하다
	let first = tup.0;
	let second = tup.1;
	let third = tup.2;
	```


- 배열
	- 여러 같은 타입의 요소를 하나로 묶음
	- rust에서 배열은 고정된 길이를 갖는다. (크지거나 작아지지 않음)
	```
	fn main() {
		let a = [1, 2, 3, 4, 5];
	}
	```