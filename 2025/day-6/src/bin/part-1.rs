use day_6::parse;

fn part_1(input: &str) -> String {
    let columns = parse(input);
    let mut count: usize = 0;

    for column in columns {
        count += compute_column(column);
    }
    count.to_string()
}

fn compute_column(column: Vec<&str>) -> usize {
    let op = column.last().unwrap();

    match op {
        &"*" => column
            .iter()
            .map_while(|part| part.parse::<usize>().ok())
            .product(),
        &"+" => column
            .iter()
            .map_while(|part| part.parse::<usize>().ok())
            .sum(),
        op => panic!("unknown operation: {op}"),
    }
}

fn main() {
    let input = include_str!("input.txt");
    let result = part_1(input);
    println!("Part 1: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input: &str = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";
        assert_eq!(part_1(input), "4277556");
    }
}
