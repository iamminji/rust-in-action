use std::rc::Rc;
use std::cell:Refcell;

#[derive(Debug)]
struct GroundStation {
    radio_freq: f64
}

fn main() {
    let base: Rc<RefCell<GroundStation>> = Rc::new(RefCell::new(
            GroundStation {
                radio_freq: 87.65
            }
    ));

    println!("base: {:?}", base);

    {
        /// base를 가변적으로 대여할 수 있는 새로운 범위를 도입한다.
        let mut base_2 = base.borrow_mut();
        base_2.radio_freq -= 12.34;
        println!("base_2: {:?}", base_2);
    }

    println!("base: {:?}", base);
    let mut base_3 = base.borrow_mut();
    base_3.radio_freq += 43.21;

    println!("base: {:?}", base);
    println!("base_3: {:?}", base_3);

}
