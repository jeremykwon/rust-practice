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
```
