use std::str;

#[derive(Debug)]
struct Number {
    number: u32,
    position: i32,
    length: i32,
}

#[derive(Debug)]
struct Symbol {
    poistion: i32
}

fn part_1(input: &str) -> String {
    let mut symbols: Vec<Vec<Symbol>> = vec![];
    let mut numbers: Vec<Vec<Number>> = vec![];
    for line in input.lines() {
        let mut line_symbols = vec![];
        let mut line_numbers = vec![];
        let mut line_position: usize = 0;
        let mut line_iter = line.chars().peekable();
        while let Some(c) = line_iter.next() {
            if c.is_ascii_digit() {
                let number_start = line_position;
                line_position += c.len_utf8();
                while let Some(c) = line_iter.peek() {
                    if c.is_ascii_digit() {
                        line_position += c.len_utf8();
                        line_iter.next();
                    } else {
                        break;
                    }
                }
                let n_str: &str = &line[number_start..line_position];
                let n = n_str.parse().unwrap();
                line_numbers.push(Number { number: n, position: number_start as i32, length: (line_position - number_start) as i32});
            } else if c == '.' {
                line_position += c.len_utf8();
            } else {
                line_symbols.push(Symbol{poistion: line_position as i32});
                line_position += c.len_utf8();
            }
        }
        symbols.push(line_symbols);
        numbers.push(line_numbers);
    }

    numbers.iter().enumerate().map(|(line_number, numbers)|{
        numbers.iter().fold(0, |sum, n| {
            let left = n.position - 1;
            let right = n.position + n.length + 1;
            let above = if line_number != 0 {
                symbols.get(line_number - 1).unwrap().iter().any(|s| s.poistion >= left && s.poistion < right)
            } else {
                false
            };
            let on = symbols.get(line_number).unwrap().iter().any(|s| s.poistion >= left && s.poistion < right);
            let under = symbols.get(line_number + 1).map(|sl| sl.iter().any(|s| s.poistion >= left && s.poistion < right)).is_some_and(|v|v);
            if above || on || under {
                sum + n.number
            } else {
                sum
            }
        })
    }).sum::<u32>().to_string()
}

fn main() {
    let input = include_str!("input.txt");
    let result = part_1(input);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(part_1(input), "4361");
    }
}
