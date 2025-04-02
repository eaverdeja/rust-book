fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);

    let a = [1, 2, 3];
    let doubled = a.iter().map(|x| x * 2).collect::<Vec<_>>();

    assert_eq!(doubled, vec![2, 4, 6]);
}
