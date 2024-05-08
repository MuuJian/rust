fn main() {
    let (a, mut b): (bool,bool) = (true, false);
    // a = true,不可变; b = false，可变
    println!("a = {}, b = {:?}", a, b);

    b = true;
    assert_eq!(a, b);
    let v = vec![1, 2, 3];

    let s = format!("{v:?}");
    println!("{}", s);
}