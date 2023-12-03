use std::fs;

fn is_valid_position(size: i32, pos: (i32, i32)) -> bool {
    return match pos {
        (x, ..) if x < 0 || x == size => false,
        (.., y) if y < 0 || y == size => false,
        _ => true
    }
}

fn is_symbol(lines: &Vec<&str>, pos: (i32, i32)) -> bool {
    let c = lines.get(pos.1 as usize).unwrap().chars().nth(pos.0 as usize).unwrap();

    return !c.is_numeric() && c != '.';
}

fn check_surroundings(lines: &Vec<&str>, pos: (i32, i32)) -> bool {
    let offsets = vec![(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];

    for offset in offsets {
        let new_pos = (pos.0 + offset.0, pos.1 + offset.1);

        if is_valid_position(lines.len() as i32, new_pos) && is_symbol(lines, new_pos) {
            return true;
        }
    }
    return false;
}

fn parse_line(lines: &Vec<&str>, numbers: &mut Vec<u32>, y: usize) {
    let mut x: usize = 0;
    let size = lines.len();
    let chars: Vec<char> = lines.get(y).unwrap().chars().collect();
    
    while x < size {
        if !chars.get(x).unwrap().is_numeric() {
            x += 1;
            continue;
        }

        let mut has_symbol = false;
        let mut num = String::new();

        while x < size && chars.get(x).unwrap().is_numeric() {
            if check_surroundings(lines, (x as i32, y as i32)) && !has_symbol {
                has_symbol = true; 
            }
            num.push(chars.get(x).unwrap().to_owned());
            x += 1;
        } 

        if has_symbol {
            numbers.push(num.parse::<u32>().unwrap());
        }
        x += 1;
    }
}

fn main() {
    let mut numbers: Vec<u32> = Vec::new();
    let file = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = file.lines().collect();

    for y in 0..lines.len() {
        parse_line(&lines, &mut numbers, y);
    }

    print!("{}", numbers.iter().sum::<u32>());
}