fn main() {
    let values = include_str!("input").lines().map(|line| {
        let mut numbers = line
            .bytes()
            .filter(|b| b.is_ascii_digit())
            .map(|b| (b - b'0') as u32);
        10 * numbers.clone().next().unwrap() + numbers.next_back().unwrap()
    });
    println!("Part 1: {}", values.sum::<u32>());
}
