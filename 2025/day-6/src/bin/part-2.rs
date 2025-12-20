use day_6::parse2;

fn part_2(input: &str) -> String {
    let (headers, values) = parse2(input);
    let mut count = 0;

    for (column_index, op) in headers.iter().enumerate() {
        let column = &values[column_index];
        let len = column[0].len();
        let mut acc: usize = match op {
            b'*' => 1,
            b'+' => 0,
            _ => panic!("ups"),
        };

        for i in 0..len {
            let mut str = String::new();
            column.iter().for_each(|&s| {
                let c = s.chars().nth(i).unwrap();
                if c.is_ascii_digit() {
                    str.push(c);
                }
            });
            let n: usize = str.parse().unwrap();
            match op {
                b'*' => acc *= n,
                b'+' => acc += n,
                _ => panic!("ups"),
            }
        }

        count += acc;
    }

    count.to_string()
}

fn main() {
    let input = include_str!("input.txt");
    let result = part_2(input);
    println!("Part 2: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input: &str = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";
        assert_eq!(part_2(input), "3263827");
    }
}
