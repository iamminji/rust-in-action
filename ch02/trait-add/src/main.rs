use std::ops::Add;
use std::time::Duration;

// T는 제너릭으로 std::ops::Add를 반드시 구현해야 함을 의미한다.
// 일종의 러스트의 연산자 오버로딩인 셈
fn add<T: Add<Output = T>>(i: T, j: T) -> T {
    i + j
}

fn main() {
    let floats = add(1.2, 3.4);
    let ints = add(10, 20);
    let durations = add(Duration::new(5, 0), Duration::new(10, 0));

    println!("{}", floats);
    println!("{}", ints);
    println!("{:?}", durations);
}
