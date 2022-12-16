#[derive(Debug)] // println!으로 File을 출력할 수 있도록 한다. std::fmt::Debug 트레이트는 매크로 내에서 {:?}과 연계하여 File을 출력 가능한 문자열로 바꾼다.
struct File {
    name: String,
    data: Vec<u8>, // Vec<u8>을 사용하면 동적 크기 조정과 같은 몇 가지 유용한 편의 기능에 접근할 수 있으므로 파일에 쓰기 작업을 시뮬레이션 할 수 있다.
}

fn main() {
    let f1 = File {
        name: String::from("f1.txt"), // String::from은 슬라이스인 문자열 리터럴에서 소유한 문자열을 생성한다.
        data: Vec::new(), // vec! 매크로가 빈 파일을 시뮬레이트한다.
    };

    // 필드에 접근하려면 . 연산자를 사용한다. 참조로 필드에 접근하면 이동 후 사용하는 문제를 피할 수 있다.
    let f1_name = &f1.name;
    let f1_length = &f1.data.len();

    println!("{:?}", f1);
    println!("{} is {} bytes long", f1_name, f1_length);
}