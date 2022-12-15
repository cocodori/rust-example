use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    // File 객체를 만들 때 경로 인자가 필요하며 파일이 존재하지 않을 경우 오류가 발생한다.
    // 이 경우 readme.md가 존재하지 않는다면 강제 종료된다.
    let f = File::open("readme.md").unwrap();
    let mut reader = BufReader::new(f);

    // 하나의 String 객체를 프로그램 수명 내내 재활용한다.
    let mut line = String::new();

    loop {
        // 디스크 읽기가 실패할 수 있으니 이를 명시적으로 처리할 필요가 있다.
        // 이 경우 실패할 때 프로그램을 강제 종료한다.
        let len = reader.read_line(&mut line).unwarp();

        if len == 0 {
            break
        }

        println!("{} ({} bytes long)", line, len);

        // 해당 String 객체의 길이를 0으로 줄인다. 다음 반복에서 기존에 있는 값이 재사용되는 것을 막는다.
        line.truncate(0);
    }
}