pub fn parse(input: &str) -> Vec<Vec<&str>> {
    let input = input.trim();
    let mut columns = vec![];
    let mut iter = input.lines();
    // First line
    for line in iter.next().unwrap().split_whitespace() {
        columns.push(vec![line]);
    }

    for line in iter {
        for (index, part) in line.split_whitespace().enumerate() {
            columns[index].push(part);
        }
    }
    columns
}

pub fn parse2(input: &str) -> (Vec<u8>, Vec<Vec<&str>>) {
    let lines: Vec<&str> = input.lines().collect();
    let header_line = lines.last().unwrap();
    let mut index: usize = 1;
    let mut start: usize = 0;
    let mut headers: Vec<(u8, usize, usize)> = vec![];
    loop {
        if index >= header_line.len() {
            break;
        }
        if header_line.as_bytes()[index] == b'*' || header_line.as_bytes()[index] == b'+' {
            headers.push((header_line.as_bytes()[start], start, index - 1));
            start = index;
        }
        index += 1;
    }
    headers.push((header_line.as_bytes()[start], start, index));

    let mut values: Vec<Vec<&str>> = headers.iter().map(|_| vec![]).collect();

    for line in lines[0..(lines.len() - 1)].iter() {
        headers
            .iter()
            .enumerate()
            .for_each(|(index, (_, start, end))| values[index].push(&line[*start..*end]));
    }

    (headers.iter().map(|v| v.0).collect(), values)
}

pub fn print_columns(columns: Vec<Vec<&str>>) {
    for column in columns {
        for part in column {
            print!("{part} ");
        }
        println!()
    }
}
