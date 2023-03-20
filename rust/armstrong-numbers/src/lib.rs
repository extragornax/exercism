pub fn is_armstrong_number(num: u32) -> bool {
    let num_str = num.to_string();

    let sum = num_str
        .chars()
        .map(|ch| ch.to_digit(10).unwrap())
        .map(|digit| digit.pow(num_str.len() as u32))
        .sum();

    num == sum
}
