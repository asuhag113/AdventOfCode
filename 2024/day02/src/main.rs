fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    part_one(&input);
    part_two(&input);
}

fn part_one(input: &String) {
    println!(
        "{}",
        input.split("\n").filter(|l| 
            {l.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>().windows(2).try_fold(0, |acc, window| {
                if acc <= 0 && [-1, -2, -3].contains(&(window[0] - window[1])) {
                    return Ok(window[0] - window[1]);
                }
                if acc >= 0 && [1, 2, 3].contains(&(window[0] - window[1])) {
                    return Ok(window[0] - window[1]);
                }
                return Err(());
            }).is_ok()}
        ).collect::<Vec<_>>().len()
    );
}

fn is_safe(input: Vec<i32>) -> bool {
    input.windows(2).try_fold(0, |acc, window| {
        if acc <= 0 && [-1, -2, -3].contains(&(window[0] - window[1])) {
            return Ok(window[0] - window[1]);
        }
        if acc >= 0 && [1, 2, 3].contains(&(window[0] - window[1])) {
            return Ok(window[0] - window[1]);
        }
        return Err(());
    }).is_ok()}

fn part_two(input: &String) {
    println!(
        "{}",
        input.split("\n").filter(|l| 
            {
                let report = l.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>();
                if is_safe(report.clone()) {
                    return true;
                }
                // I think we can further optimize by only checking "unsafe" indices with some preprocessing
                (0..report.len()).any(|i|{
                    [&report[0..i], &report[i+1..]].concat().windows(2).try_fold(0, |acc, window| {
                    if acc <= 0 && [-1, -2, -3].contains(&(window[0] - window[1])) {
                        return Ok(window[0] - window[1]);
                    }
                    if acc >= 0 && [1, 2, 3].contains(&(window[0] - window[1])) {
                        return Ok(window[0] - window[1]);
                    }
                    return Err(());
                }).is_ok()})}
        ).collect::<Vec<_>>().len()
    );
}
