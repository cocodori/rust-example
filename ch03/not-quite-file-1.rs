#![allow(unused_variables)] // 컴파일러 경고 완화

type File = String; // 타입 별칭. 컴파일러는 String과 File을 구분하지 ㅇ낳지만 소스 코드에서는 구분한다.

fn open(f: &mut File) -> bool {
    true // 함수가 항상 성공한다고 가정
}

fn close(f: &mut File) -> bool {
    true // 함수가 항상 성공한다고 가정
}

#[allow(dead_code)] // 사용하지 않는 함수에 대한 컴파일러 경고 완화
fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! { // ! 반환 타입은 이 함수가 절대 어떤 값도 반환하지 않는다고 컴파일러에게 알리는 역할
    unimplemented!() // 프로그램이 이 지점에 오게 되면 중단시키는 매크로
}

fn main() {
    let mut f1 = File::from("f1.txt"); // 3행의 타입 선언으로 File은 String의 모든 메서드를 상속한다.
    open(&mut f1);
    // read(f1, vec![]);
    close(&mut f1);
}