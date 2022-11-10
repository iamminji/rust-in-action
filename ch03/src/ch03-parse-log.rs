// 자동 생성 코드를 통해 이 열거형을 화면에 출력한다.
#[derive(Debug)]
enum Event {
    Update,
    Delete,
    Unknown,
}

// 이 크레이트 컨텍스트에서 사용될 String의 편리한 이름이다.
type Message = String;

fn parse_log(line: &'static str) -> (Event, Message) {
    let parts: Vec<&str> = line.splitn(2, ' ').collect();

    if parts.len() == 1 {
        return (Event::Unknown, String::from(line));
    }

    let event = parts[0];
    let rest = String::from(parts[1]);

    match event {
        "UPDATE" | "update" => (Event::Update, rest),
        "DELETE" | "delete" => (Event::Delete, rest),
        _ => (Event::Unknown, String::from(line)),
    }
}

fn main() {
    let log = "BEGIN Transaction XK342
UPDATE 234:LS/32231 blah blah
DELETE 342:L0/22111";

    for line in log.lines() {
        let parse_result = parse_log(line);
        println!("{:?}", parse_result);
    }
}
