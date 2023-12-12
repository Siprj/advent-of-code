fn parse(input: &str) -> Vec<(Vec<char>, Vec<usize>)> {
    input.lines().map(|l|{
        let (gears, counts) = l.split_once(' ').unwrap();
        let counts = counts.split(',').map(|s| s.parse::<usize>().unwrap()).collect();

        (gears.chars().collect::<Vec<char>>(),counts)
    }).collect()
}

fn check_solution(gears: &[char], checks: &[usize]) -> bool {
    let mut check_index = 0;
    for group in gears.split(|c| c == &'.') {
        let count = group.len();
        if count != 0 {
            if check_index != checks.len() && count == checks[check_index] {
                check_index += 1;
            } else {
                return false;
            }
        }
    }
    check_index == checks.len()
}

fn solution_count(input: &(Vec<char>, Vec<usize>)) -> usize {
    let (gears, checks) = input;
    let mut solution_count = 0;
    let mut gears = gears.clone();
    let empty_fields: Vec<usize> = gears.iter().enumerate().filter(|(_, c)| c == &&'?').map(|(i, _)|i).collect();
    for n in 0..(2usize.pow(empty_fields.len() as u32)) {
        for (i, gi) in empty_fields.iter().enumerate() {
            if (1 << i) & n == 0 {
                gears[*gi] = '#';
            } else {
                gears[*gi] = '.';
            }
        }
        if check_solution(&gears, checks) {
            solution_count += 1;
        }

    }
    solution_count
}

fn part_1(input: &str) -> String {
    let inputs = parse(input);

    let count: usize = inputs.iter().map(solution_count).sum();
    count.to_string()
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
        let input: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!(part_1(input), "21");
    }

    #[test]
    fn it_works_2() {

        let input = ("???.###".chars().collect() , vec![1,1,3]);
        assert_eq!(solution_count(&input), 1);
    }

    #[test]
    fn it_works_3() {

        let input = ("?###????????".chars().collect() , vec![3,2,1]);
        assert_eq!(solution_count(&input), 10);
    }
}
