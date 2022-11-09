// println!으로 File을 출력할 수 있도록 한다.
// std::fmt::Debug 트레이트는 매크로 내에서 {:?}과 연계하여 File을 출력 가능한 문자열로 바꾼다.
#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

fn main() {
    let f1 = File {
        name: String::from("f1.txt"),
        data: Vec::new(),
    };

    let f1_name = &f1.name;
    let f1_length = &f1.data.len();

    println!("{:?}", f1);
    println!("{} is {} bytes long", f1_name, f1_length);
}
