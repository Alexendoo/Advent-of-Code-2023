fn main() {
    let input = include_str!("input");
    let values = input.lines().map(|line| {
        let mut numbers = line
            .bytes()
            .filter(|b| b.is_ascii_digit())
            .map(|b| (b - b'0') as u32);
        10 * numbers.clone().next().unwrap() + numbers.next_back().unwrap()
    });
    println!("Part 1: {}", values.sum::<u32>());

    let values = input.lines().map(|line| {
        let mut x = (0..line.len()).map(|start| &line[start..]).filter_map(num);
        10 * x.clone().next().unwrap() + x.next_back().unwrap()
    });
    println!("Part 2: {}", values.sum::<u32>());
}

fn num(s: &str) -> Option<u32> {
    if let Some(digit) = s.chars().next()?.to_digit(10) {
        return Some(digit);
    }

    let words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    words
        .into_iter()
        .position(|word| s.starts_with(word))
        .map(|pos| pos as u32 + 1)
}
