fn main() {
    let memory = include_str!("../input");

    let q1 = parse(memory, false);
    let q2 = parse(memory, true);
    assert_eq!(q1, 153469856);
    assert_eq!(q2, 77055967);
    print!("{} {}\n", q1, q2);
}

fn parse(memory: &str, is_part_2: bool) -> u32 {
    let mut enabled = true;
    let mut total = 0;
    let mut i = 0;
    while i < memory.len() - 6 {
        if memory[i..].starts_with("do()") {
            enabled = true;
            i += 4;
            continue;
        }
        if memory[i..].starts_with("don't()") {
            enabled = false;
            i += 7;
        }
        if is_part_2 && !enabled {
            i += 1;
            continue;
        }
        if &memory[i..i + 4] != "mul(" {
            i += 1;
            continue;
        }
        if !&memory[i + 4..].contains(')') {
            i += 1;
            continue;
        }
        let Some(parts) = memory[i + 4..].split_once(')') else {
            i += 1;
            continue;
        };
        let Some(params) = parts.0.split_once(',') else {
            i += 1;
            continue;
        };
        let (Some(l), Some(r)): (Option<u32>, Option<u32>) =
            (params.0.parse().ok(), params.1.parse().ok())
        else {
            i += 1;
            continue;
        };
        total += l * r;
        i += 1;
    }
    total
}
