use std::time::{Duration, Instant};

/*
    지속 시간에 도달하면 반복을 중지
*/
fn main() {
    let mut count = 0;
    let time_limit = Duration::new(1,0); // 1초를 나타내는 Duration
    let start = Instant::now();

    // Instatnt에서 Duration을 빼면 Duration이 반환된다.
    while (Instant::now() - start) < time_limit {
        count += 1;
    }

    println!("{}", count);
}