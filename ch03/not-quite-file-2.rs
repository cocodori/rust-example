#![allow(unused_variables)] // 경고를 내지 않는다.

#[derive(Debug)] //File이 println!과 fmt! 매크로와 함께 동작하도록 한다.
struct File {
    name: String,
    data: Vec<u8>,
}

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

fn read(
    f: &File,
    save_to: &mut Vec<u8>
) -> usize { // 읽은 바이트의 수를 반환한다
    let mut tmp = f.data.clone(); // save_to.append()로 인해 입력값 Vec<T>가 줄어들기 때문에 data의 복사본을 만든다.
    let read_length = tmp.len();

    save_to.reserve(read_length); // 데이터를 저장할 공간이 충분한지 확인한다.
    save_to.append(&mut tmp); // f의 내용을 담기 위해 save_to 버퍼에 충분한 데이터를 할당한다.
    read_length
}

fn main() {
    let mut f2 = File {
        name: String::from("2.txt"),
        data: vec![114, 117, 115, 116, 33],
    };

    let mut buffer: Vec<u8> = vec![];

    // 파일과 상호 작용한다
    open(&mut f2);
    let f2_length = read(&f2, &mut buffer);
    close(&mut f2);

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f2);
    println!("{} is {} bytes long", &f2.name, f2_length);
    println!("{}", text); // 바이트를 실제 단어로 표시
}