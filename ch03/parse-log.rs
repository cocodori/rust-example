#[derive(Debug)] // 자동생성 코드를 통해 이 열거형을 화면에 출력한다.
enum Event { // 인식할 수 없는 이벤트에 대한 값을 포함하여 세 가지 Event 열거값 생성
    Update,
    Delete,
    Unknown
}

// 이 크레이트 문맥에서 사용할 String의 편리한 이름
type Message = String;

// 행을 분석해 반구조화된 데이터로 변환하는 함수
fn parse_log(line: &'static str) -> (Event, Message) {
    // Vec<_>는 요소의 타입을 추론하라고 러스트에 요청한다.
    let parts: Vec<&str> =
        line.splitn(2, ' ')
            .collect(); // collect()는 line.splitn()에서 생성된 반복자를 써서 Vec<T>를 반환한다.
    //line.splitn()이 로그를 두 부분으로 나누지 못한다면 오류를 반환한다.
    if parts.len() == 1 {
        return (Event::Unknown, String::from(line))
    }

    let event = parts[0];
    let rest = String::from(parts[1]);

    // 알려진 이벤트의 경우 구조화된 데이터를 반환
    // 이벤트 타입을 모르면 전체 행을 반환
    match event {
        "UPDATE" | "update" => (Event::Update, rest),
        "DELETE" | "delete" => (Event::Delete, rest),
        _ => (Event::Unknown, String::from(line)),
    }
}

fn main() {
    let log = "BEGIN Transaction XK342
UPDATE 234:LS/32231 {\"price\": 31.00} -> {\"price\":40.00}
DELETE 342:LO/22111";

    for line in log.lines() {
        let parse_result = parse_log(line);
        println!("{:?}", parse_result);
    }
}