#[macro_export]
macro_rules! instrument {
    ($part1:expr, $part2:expr) => {
        let mut now = std::time::Instant::now();
        println!("Part 1: {}", $part1);
        println!("(elapsed: {:?})", now.elapsed());
        now = std::time::Instant::now();
        println!("");
        println!("Part 2: {}", $part2);
        println!("(elapsed: {:?})", now.elapsed());
    };
}
