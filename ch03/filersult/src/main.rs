use rand::prelude::*; // rand 크레이트에서 공통 트레이트와 타입을 현재 크레이트 범위로 가져온다.

fn one_in(denominator: u32) -> bool { // 산발적인 오류를 일으키는 도움 함수
    thread_rng().gen_ratio(1, denominator) // thread_rng()는 스레드 로컬 난수 생성기를 만든다. gen_ratio(n,m)은 n/m 확률을 가지는 불값을 반환
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new()
        }
    }
    
    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    fn read(
        self: &File,
        save_to: &mut Vec<u8>
    ) -> Result<usize, String> { // String을 사용하여 임의의 오류 메시지를 표시한다
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Ok(read_length) // 이 코드에서 read()는 절대 실패하지 않는다. 하지만 Result 타입을 반환할 것이므로 read_length를 Ok로 감싼다.
    }
} 

fn open(f: File) -> Result<File, String> {
    if one_in(10_000) { //1만번에 한 번 꼴로 오류를 반환한다.
        let err_msg = String::from("Permission denied");
        return Err(err_msg);
    }

    Ok(f)
}

fn close(f: File) -> Result<File, String> {
    if one_in(100_000) {
        let err_msg = String::from("Interrupted by signal!");
        return Err(err_msg);
    }

    Ok(f)
}

fn main() {
    let f4_data: Vec<u8> = vec![114, 117, 115, 116, 33];
    let mut f4 = File::new_with_data("4.txt", &f4_data);

    let mut buffer: Vec<u8> = vec![];

    // Ok로부터 T를 풀어 T를 남긴다.
    f4 = open(f4).unwrap();
    let f4_length = f4.read(&mut buffer).unwrap();
    f4 = close(f4).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f4);
    println!("{} is {} bytes long", &f4.name, f4_length);
    println!("{}", text);
}
