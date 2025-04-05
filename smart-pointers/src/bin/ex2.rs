use std::rc::Rc;
fn main() {
    let r1 = Rc::new(0);
    let r4 = {
        let r2 = Rc::clone(&r1);
        println!("{} {}", Rc::strong_count(&r1), Rc::weak_count(&r1));
        Rc::downgrade(&r2)
    };
    // r4 is our weak ref - r2 has already gone out of scope
    println!("{} {}", Rc::strong_count(&r1), Rc::weak_count(&r1));
    // 1 1

    let r5 = Rc::clone(&r1);
    println!("{} {}", Rc::strong_count(&r1), Rc::weak_count(&r1));
    // 2 1

    // Note: r4 still counts as a weak ref despite being upgraded
    // and incrementing the strong count with r6
    let r6 = r4.upgrade();
    println!("{} {}", Rc::strong_count(&r1), Rc::weak_count(&r1));
    // 3 1
}
