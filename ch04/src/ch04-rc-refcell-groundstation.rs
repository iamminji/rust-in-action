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
}
