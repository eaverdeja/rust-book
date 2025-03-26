use std::collections::HashMap;

fn median(v: &[i32]) -> Option<i32> {
    if v.is_empty() {
        return None;
    }

    let mut sorted = v.to_vec();
    sorted.sort();

    let mid = v.len() / 2;

    if v.len() % 2 == 0 {
        Some((sorted[mid] + sorted[mid - 1]) / 2)
    } else {
        Some(sorted[mid])
    }
}

fn mode(v: &[i32]) -> Option<i32> {
    let mut map = HashMap::new();
    for &i in v {
        *map.entry(i).or_insert(0) += 1;
    }
    map.into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(num, _)| num)
}

fn main() {
    let v = vec![5, 1, 1, 3, 2];

    let median = median(&v);
    match median {
        Some(m) => println!("Median is: {m}"),
        None => println!("Empty array!"),
    }

    let mode = mode(&v);
    match mode {
        Some(m) => println!("Mode is: {m}"),
        None => println!("Empty array!"),
    }
}
