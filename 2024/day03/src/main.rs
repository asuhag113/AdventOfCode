use regex::Regex;

fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    part_one(&input);
    part_two(&input);
}

fn part_one(input: &String) {
    println!(
        "{}",
        Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")
            .unwrap()
            .captures_iter(input)
            .map(|c| c.extract::<2>())
            .map(|(_, [x, y])| x.parse::<u32>().unwrap() * y.parse::<u32>().unwrap())
            .sum::<u32>()
    );
}

fn part_two(input: &String) {
    println!(
        "{}",
        Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")
            .unwrap()
            .captures_iter(
                &input
                    .split("don't()")
                    .enumerate()
                    // code starts off as enabled
                    .map(|(idx, el)| if idx == 0 {
                        el.to_string()
                    } else {
                        el.split("do()").skip(1).collect::<Vec<_>>().join("")
                    })
                    .collect::<Vec<_>>()
                    .join("")
            )
            .map(|c| c.extract::<2>())
            .map(|(_, [x, y])| x.parse::<u32>().unwrap() * y.parse::<u32>().unwrap())
            .sum::<u32>()
    );
}
