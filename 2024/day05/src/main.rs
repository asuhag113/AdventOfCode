fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    part_one(&input);
    // part_two(&input);
}

fn part_one(input: &String) {
    let mut page_to_invalid = std::collections::HashMap::new();
    input.split("\n").take_while(|l| *l != "").for_each(|r| {
        let parts = r.split("|").collect::<Vec<_>>();
        let (before, after) = (parts[0], parts[1]);
        page_to_invalid.entry(before).or_insert(vec![]).push(after);
    });
    let mut invalid_values = Vec::new();
    println!(
        "{}",
        input
            .split("\n")
            .skip_while(|l| *l != "")
            .skip(1)
            .filter(|r| {
                invalid_values.clear();
                r.split(",").collect::<Vec<_>>().iter().rev().all(|c| {
                    println!("c {}", c);
                    let valid = !invalid_values.contains(&c);
                    if page_to_invalid.get(c).is_some() {
                        invalid_values.extend(page_to_invalid.get(c).unwrap())
                    }
                    return valid;
                })
            })
            .map(
                |r| r.split(",").collect::<Vec<_>>()[r.split(",").collect::<Vec<_>>().len() / 2]
                    .parse::<u32>()
                    .unwrap()
            )
            .sum::<u32>()
    );
}

fn part_two(input: &String) {
    let mut page_to_invalid = std::collections::HashMap::new();
    input.split("\n").take_while(|l| *l != "").for_each(|r| {
        let parts = r.split("|").collect::<Vec<_>>();
        let (before, after) = (parts[0], parts[1]);
        page_to_invalid.entry(before).or_insert(vec![]).push(after);
    });
    let mut invalid_values = Vec::new();
    let invalid_updates = input
        .split("\n")
        .skip_while(|l| *l != "")
        .skip(1)
        .filter(|r| {
            invalid_values.clear();
            r.split(",").collect::<Vec<_>>().iter().rev().all(|c| {
                let valid = !invalid_values.contains(&c);
                if page_to_invalid.get(c).is_some() {
                    invalid_values.extend(page_to_invalid.get(c).unwrap())
                }
                return valid;
            })
        });
    invalid_updates.for_each(|u| {
        let mut update_to_index = std::collections::HashMap::new();
        u.split(",")
            .collect::<Vec<_>>()
            .iter()
            .enumerate()
            .for_each(|(i, c)| {
                update_to_index.insert(c, i);
            })
    });
}
