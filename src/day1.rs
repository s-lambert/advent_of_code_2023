fn find_all_digits(input: &str) -> Vec<u32> {
    let number_words = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];
    let mut digits = Vec::new();
    let mut position = 0;

    while position < input.len() {
        let mut found = false;

        // Check for number words
        for (word, number) in number_words.iter() {
            if input[position..].starts_with(word) {
                digits.push(*number);
                position += 1;
                found = true;
                break;
            }
        }

        // Check for numeric digits
        if !found {
            if let Some(next_char) = input[position..].chars().next() {
                if next_char.is_digit(10) {
                    if let Some(digit) = next_char.to_digit(10) {
                        digits.push(digit);
                    }
                    position += next_char.len_utf8();
                    continue;
                }
            }
            position += 1;
        }
    }

    digits
}

pub fn results() {
    let day_1 = include_str!("day1.txt").lines();

    let part_1 = day_1
        .clone()
        .map(|line| {
            let digits: Vec<i32> = line
                .chars()
                .filter(|c| c.is_digit(10))
                .filter_map(|c| c.to_digit(10))
                .map(|d| d as i32)
                .collect();

            if digits.len() == 0 {
                return 0;
            }

            let first_digit = digits.first().unwrap();
            let last_digit = digits.last().unwrap();

            (first_digit * 10) + last_digit
        })
        .sum::<i32>();

    let part_2 = day_1
        .clone()
        .map(|line| {
            let digits = find_all_digits(line);

            if digits.len() == 0 {
                println!("failed {}", line);
            }

            let first_digit = digits.first().unwrap();
            let last_digit = digits.last().unwrap();

            (first_digit * 10) + last_digit
        })
        .sum::<u32>();

    println!("part1: {}", part_1);
    println!("part2: {}", part_2);
}
