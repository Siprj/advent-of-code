use num::traits::clamp_max;

fn read_while<F>(str: &str, f: F) -> usize
where
    F: Fn(char) -> bool,
{
    for (i, c) in str.chars().enumerate() {
        if !f(c) {
            return i;
        }
    }
    str.len()
}

fn parse(input: &str) -> Vec<(u64, u64)> {
    let len = input.len();
    let mut i: usize = 0;
    let mut result = vec![];
    let mut count = true;
    while i < len {
        let upper = i + 4;
        if upper >= len {
            break;
        }
        if &input[i..upper] == "mul(" {
            i = upper;

            let upper = clamp_max(i + 4, len);
            if upper <= i {
                break;
            }
            let slice = &input[i..upper];
            let read = read_while(slice, |c| c.is_ascii_digit());
            if read == 0 {
                continue;
            }
            let n1 = input[i..i + read].parse::<u64>().unwrap();
            i += read;

            let upper = clamp_max(i + 1, len);
            if upper <= i {
                break;
            }
            let comma: &str = &input[i..upper];
            i = upper;
            if comma != "," {
                continue;
            }

            let upper = clamp_max(i + 4, len);
            if upper <= i {
                break;
            }
            let slice = &input[i..upper];
            let read = read_while(slice, |c| c.is_ascii_digit());
            if read == 0 {
                continue;
            }
            let n2 = input[i..i + read].parse::<u64>().unwrap();
            i += read;

            let upper = clamp_max(i + 1, len);
            if upper <= i {
                break;
            }
            let right_one: &str = &input[i..upper];
            i = upper;
            if right_one != ")" {
                continue;
            }

            if count {
                result.push((n1, n2));
            }

            continue;
        }

        let upper = i + 4;
        if upper >= len {
            break;
        }
        if &input[i..upper] == "do()" {
            count = true;
            i = upper;
            continue;
        }

        let upper = i + 7;
        if upper >= len {
            break;
        }
        if &input[i..upper] == "don't()" {
            count = false;
            i = upper;
            continue;
        }

        i += 1;
    }
    result
}

fn part_2(input: &str) -> String {
    let lines = parse(input);
    let result: u64 = lines.iter().map(|(n1, n2)| n1 * n2).sum();
    result.to_string()
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
        let input: &str =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part_2(input), "48");
    }
}
