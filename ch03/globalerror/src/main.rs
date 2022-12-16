use rand::{random}; // rand 크레이트를 지역 범위로 가져온다.

static mut ERROR: isize = 0; // ERROR를 0으로 초기화한다.

struct File; // 실험할 구조체로 크기가 0인 타입을 만든다.

#[allow(unused_variables)]
fn read(f: &File, save_to: &mut Vec<u8>) -> usize {
    if random() && random() && random() { // 이 조건식이 실행될 때 여덟 번 중 한 번은 참을 반환한다.
        unsafe {
            ERROR = 1; // ERROR를 1로 지정. 시스템 나머지 부분에 오류가 발생했음을 알린다.
        }
    }

    0 // 항상 0 바이트를 읽은 것으로 간주한다.
}

#[allow(unused_mut)]
fn main() {
    let mut f = File;
    let mut buffer = vec![];

    read(&f, &mut buffer);
    unsafe {
        if ERROR != 0 {
            panic!("An error has occured!!!!!!!!")
        }
    }
}