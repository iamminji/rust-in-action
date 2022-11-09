// i: &'a i32 는
// 수명 변수 'a를 i의 수명으로 바인드 한다.
// 이 구문은 매개 변수 i는 수명 a를 가지는 i32 타입의 참조이다.
//
// 수명 시스템은 컴파일러가 대부분 자체적으로 추론할 수 있긴 하지만
// 가끔 프로그래머에게 명시적으로 지정해 줄것을 요청하기도 한다.
fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    // i와 j 변수에 담긴 값을 역참조하여 더한다.
    *i + *j
}

fn main() {
    let a = 10;
    let b = 10;

    let res = add_with_lifetimes(&a, &b);

    println!("{}", res);
}
