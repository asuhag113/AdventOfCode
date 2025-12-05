fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    part_one(&input);
    part_two(&input);
}

fn get_next_char(prev_char: char) -> Option<char> {
    match prev_char {
        'X' => Some('M'),
        'M' => Some('A'),
        'A' => Some('S'),
        'S' => Some('#'),
        _ => None,
    }
}

fn search(grid: &Vec<Vec<char>>, i: usize, j: usize, dir: (i32, i32)) -> u32 {
    let mut prev_char = 'X';
    let mut next_char = get_next_char(prev_char).unwrap();
    let (mut ni, mut nj) = (i as i32 + dir.0, j as i32 + dir.1);
    while (ni >= 0 && ni < grid.len() as i32) && (nj >= 0 && nj < grid[0].len() as i32) {
        if grid[ni as usize][nj as usize] != next_char {
            return 0;
        }
        prev_char = next_char;
        next_char = get_next_char(prev_char).unwrap();
        ni += dir.0;
        nj += dir.1;
        if next_char == '#' {
            return 1;
        }
    }
    return 0;
}

fn part_one(input: &String) {
    let grid = input
        .split("\n")
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut res = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] != 'X' {
                continue;
            }
            res += [
                search(&grid, i, j, (0, 1)),
                search(&grid, i, j, (0, -1)),
                search(&grid, i, j, (1, 0)),
                search(&grid, i, j, (-1, 0)),
                search(&grid, i, j, (1, 1)),
                search(&grid, i, j, (1, -1)),
                search(&grid, i, j, (-1, 1)),
                search(&grid, i, j, (-1, -1)),
            ]
            .iter()
            .sum::<u32>();
        }
    }
    println!("{}", res);
}

fn search_x(grid: &Vec<Vec<char>>, i: usize, j: usize) -> u32 {
    if i as i32 - 1 < 0 || j as i32 - 1 < 0 || i + 1 >= grid.len() || j + 1 >= grid[0].len() {
        return 0;
    }
    let binding = vec![grid[i - 1][j - 1], grid[i][j], grid[i + 1][j + 1]];
    let mut l_to_r = binding.iter().collect::<Vec<_>>();
    l_to_r.sort();
    let l = String::from_iter(l_to_r);
    let binding = vec![grid[i + 1][j - 1], grid[i][j], grid[i - 1][j + 1]];
    let mut r_to_l = binding.iter().collect::<Vec<_>>();
    r_to_l.sort();
    let r = String::from_iter(r_to_l);
    if l == "AMS" && r == "AMS" {
        return 1;
    }
    return 0;
}

fn part_two(input: &String) {
    let grid = input
        .split("\n")
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut res = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] != 'A' {
                continue;
            }
            res += search_x(&grid, i, j);
        }
    }
    println!("{}", res);
}
