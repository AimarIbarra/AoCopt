use std::{
    hint::black_box,
    io::{stdin, Read},
    time::Instant,
};

fn solution(input: &str) -> u32 {
    let (first, last) = input
        .as_bytes()
        .split(|&b| b == b'\n')
        .map(|line| {
            let mut iter = line.iter().copied();
            let first = iter.find(|&b| b <= b'9').unwrap_or(0);
            let last = iter.rfind(|&b| b <= b'9').unwrap_or(first);
            ((first & 0xf) as u32, (last & 0xf) as u32)
        })
        .fold((0, 0), |(first, last), (f, l)| (first + f, last + l));
    first * 10 + last
}

fn main() {
    let mut buf = String::with_capacity(4096);
    stdin().read_to_string(&mut buf).unwrap();
    _ = solution(black_box(&buf));
    let start = Instant::now();
    let res = solution(&buf);
    let took = start.elapsed().as_nanos();
    println!("{res}\n{took}")
}
