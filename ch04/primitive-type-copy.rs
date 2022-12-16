fn use_value(_val: i32) { // use_value()는 _val 인자의 소유권을 가진다. 
}

fn main() {
    let a = 123;
    use_value(a);

    println!("{}", a); // use_value()의 실행이 끝난 후에 a에 접근하는 것은 적법하다.
}