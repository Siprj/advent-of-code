
#[derive(Debug)]
struct Number {
    number: u32,
    right: i32,
    left: i32,
}

#[derive(Debug)]
struct Symbol {
    poistion: i32
}

fn part_2(input: &str) -> String {
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
                line_numbers.push(Number { number: n, left: number_start as i32, right: line_position as i32});
            } else if c == '.' {
                line_position += c.len_utf8();
            } else if c == '*' {
                line_symbols.push(Symbol{poistion: line_position as i32});
                line_position += c.len_utf8();
            } else {
                line_position += c.len_utf8();
            }

        }
        symbols.push(line_symbols);
        numbers.push(line_numbers);
    }

    symbols.iter().enumerate().map(|(line_number, symbols)|{
        symbols.iter().fold(0, |sum, s| {
            let mut above: Vec<&Number> = if line_number != 0 {
                numbers.get(line_number - 1).unwrap().iter().filter(|n| n.left - 1 <= s.poistion && s.poistion < n.right + 1).collect()
            } else {
                vec![]
            };
            let mut on: Vec<&Number> = numbers.get(line_number).unwrap().iter().filter(|n| n.left - 1 <= s.poistion && s.poistion < n.right + 1).collect();
            let mut under: Vec<&Number> = numbers.get(line_number + 1).map(|nl| nl.iter().filter(|n| n.left - 1 <= s.poistion && s.poistion < n.right + 1).collect()).unwrap_or(vec![]);
            let mut all = vec![];
            all.append(&mut above);
            all.append(&mut on);
            all.append(&mut under);

            if all.len() == 2 {
                sum + (all[0].number * all[1].number)
            } else {
                sum
            }
        })
    }).sum::<u32>().to_string()
}

fn main() {
    let input = include_str!("input.txt");
    let result = part_2(input);
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
        assert_eq!(part_2(input), "467835");
    }
}

