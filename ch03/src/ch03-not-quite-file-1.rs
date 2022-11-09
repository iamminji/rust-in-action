// 컴파일러 경고 완화
#![allow(unused_variables)]

type File = String;

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

// 사용하지 않는 함수에 대한 컴파일러 경고를 완화한다.
#[allow(dead_code)]
// ! 의미는 이 함수가 절대로 어떤 값도 반환하지 않는다고 러스트 컴파일러에 알려주는 역할을 한다.
fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
    // 프로그램이 이 지점에 오게 되면 중단 시키는 매크로
    unimplemented!()
}

fn main() {
    let mut f1 = File::from("f1.txt");
    open(&mut f1);
    // read(f1, vec![]);
    close(&mut f1);
}
