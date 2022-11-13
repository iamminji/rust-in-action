#![allow(unused_variables)]

#[derive(Debug)]
struct CubeSat {
    id: u64,
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

fn check_status(sat_id: CubeSat) -> StatusMessage {
    StatusMessage::Ok
}

fn main() {
    // Ownership originates here at the creation of the CuveSat object.
    let sat_a = CubeSat { id: 0 };
    let sat_b = CubeSat { id: 1 };
    let sat_c = CubeSat { id: 2 };

    // Ownership of the object moves to check_status()
    // but is not returned to main().
    let a_status = check_status(sat_a);
    let b_status = check_status(sat_b);
    let c_status = check_status(sat_c);
    println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);

    //  sat_a is no longer the owner of the object, making access invalid.
    let a_status = check_status(sat_a);
    let b_status = check_status(sat_b);
    let c_status = check_status(sat_c);

    println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);

    // 내가 이해한게 맞다면...
    // sat_a는 첫번째 check_status 를 호출한 시점에
    // main에서 check_status로 소유권이 넘어가고
    // 호출이 완료되면 수명이 끝난다.
    // 그래서 두번째 check_status를 호출할 수가 없다. (수명이 끝났으므로)
    //
    // primitive type에서 정상 동작한 이유는
    // rust에서 primitive type은 Copy 트레이트를 내부적으로 구현했기 때문이다.
}
