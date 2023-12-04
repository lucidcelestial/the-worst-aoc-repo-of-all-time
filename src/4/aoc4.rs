fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();

    // every glimmer of hope has left my body
    let result: u32 = file
        .lines()
        .map(|row| {
            row[9..row.len()].split('|').map(|x| {
                x.trim()
                    .split(" ")
                    .filter(|val| !val.is_empty())
                    .map(|num| num.parse::<u32>().unwrap())
            })
        })
        .map(|mut nums| {
            nums
                .next()
                .unwrap()
                .filter(|num| nums.clone().next().unwrap().any(|val| val == *num))
                .collect::<Vec<u32>>()
        })
        .filter(|x| x.len() != 0)
        .map(|x| match x.len() as u32 {
            1 => 1,
            x => 2_u32.pow(x - 1),
        })
        .sum();

    print!("{:?}", result);
}