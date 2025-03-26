use std::collections::HashMap;

fn median(v: &mut Vec<i32>) -> i32 {
    let mid = v.len() / 2;
    v.sort();
    v[mid]
}

fn mode(v: &mut Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for i in v {
        map.entry(*i).and_modify(|e| *e += 1).or_insert(1);
    }
    let mut mode = 0;
    let mut max_count = 0;
    for (key, count) in map {
        if count > max_count {
            mode = key;
            max_count = count;
        }
    }
    mode
}

fn main() {
    let mut v = vec![5, 1, 1, 3, 2];

    let median = median(&mut v);
    println!("{median}");

    let mode = mode(&mut v);
    println!("{mode}");
}
