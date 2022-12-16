use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct GroundStation {
    radio_freq: f64 // Mhz
}

/*
    - 다른 타입으로 감싸서 더 많은 기능을 타입에 추가하면 일반적으로 런타임 성능 저하
    - Clone 구현을 금지해야 할 만큼 비싸다면 Rc<T>는 간편한 대안.
*/
fn main() {
    let base: Rc<RefCell<GroundStation>> = Rc::new(RefCell::new(
        GroundStation {
            radio_freq: 87.65
        }
    ));

    println!("base: {:?}", base);

    // base를 가변적으로 대여할 수 있는 새로운 범위
    {
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