#![allow(unused_variables)] // 함수 내에서 사용하지 않는 변수에 대한 경고를 내지 않는다.

#[derive(Debug)]
struct File; // 스터브 File 타입을 선언한다.

trait Read { // 트레이트에 특정 이름을 지정한다.
    fn read(
        self: &Self,
        save_to: &mut Vec<u8>
    ) -> Result<usize, String>; // trait 블록은 구현체가 반드시 따라야 할 함수의 시그너처 타입을 포함한다. pseudo 타입 Self는 REad를 구현하는 타입에 대한 자리 표시자다.
}

impl Read for File {
    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        Ok(0)
    }
}

fn main() {
    let f = File{};
    let mut buffer = vec!();
    let n_bytes = f.read(&mut buffer).unwrap();
    println!("{} byte(s) read from {:?}", n_bytes, f);
}