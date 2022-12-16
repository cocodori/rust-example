#[derive(Debug)]
struct CubeSat {
    id: u64,
    mailbox: Mailbox,
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

type Message = String;

struct GroundStation;

impl GroundStation {
    fn send(
        &self,
        mailbox: &mut Mailbox,
        to: &CubeSat,
        msg: Message
    ) { // 메시지를 보내기 위해 Mailbox.post()를 호출하며 Message의 소유권을 전달한다.
        mailbox.post(to, msg);
    }
}

impl CubeSat {
    fn recv(
        &self,
        mailbox: &mut Mailbox
    ) -> Option<Message> { // 메시지를 받기 위해 Mailbox.deliver()를 호출하며 Message의 소유권을 얻는다.
        mailbox.deliver(&self)
    }
}

impl Mailbox {
    fn post(&mut self, msg: Message) { // Mailbox.post()는 자신에 대한 가변 접근과 Message에 대한 소유권을 필요로 한다.
        self.messages.push(msg);
    }

    fn deliver(
        &mut self,
        recipient: &CubeSat
    ) -> Option<Message> { // deliver()는 id 필드를 얻기 위해 CubeSat에 대한 참조가 필요하다.
        for i in 0..self.messages.len() {
            if self.messages[i].to == recipient.id {
                let msg = self.messages.remove(i);
                return Some(msg); // 메시지를 찾으면 Option 타입에 따라 Some으로 감싼 Message와 함께 early return
            }
        }

        None // 아무 메시지도 없으면 None 반환한다
    }
}

fn main() {
    let base = GroundStation {};
    let mut sat_a = CubeSat {
        id: 0,
        mailbox: Mailbox {
            messages: vec![]
        }
    };

    println!("t0: {:?}", sat_a);

    base.send(&mut sat_a, Message::from("hello there!"));

    println!("t1: {:?}", sat_a);

    let msg = sat_a.recv();
    println!("t2: {:?}", sat_a);

    println!("msg: {:?}", msg);
}