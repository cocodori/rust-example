fn use_value(_val: Demo) { // use_value()는 _val의 소유권을 가진다.
}

struct Demo {
    a: i32,
}

fn main() {
    let demo = Demo { a: 123 };
    use_value(demo);

    println!("{}", demo.a); // use_value()의 실행이 종료된 후라도 demo_a에 접근하는 것은 불가하다. 
}