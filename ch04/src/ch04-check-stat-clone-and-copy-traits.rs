#[derive(Debug, Clone, Copy)]
struct CubeSat {
    id: u64,
}

// examples
// impl Copy for CubeSat { }
// imple Clone for CubeSat {
//      fn clone(&self) -> self {
//          CubeSat { id: self.id }
//      }
// }

#[derive(Debug, Clone, Copy)]
enum StatusMessage {
    Ok,
}

fn check_status(sat_id: CubeSat) -> StatusMessage {
    StatusMessage::Ok
}

fn main() {
    let sat_a = CubeSat { id: 0 };

    let a_status = check_status(sat_a.clone());
    println!("a: {:?}", a_status.clone());

    let a_status = check_status(sat_a);
    println!("a: {:?}", a_status);
}
