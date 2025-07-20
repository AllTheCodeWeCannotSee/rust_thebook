fn main() {
    let v = vec![1, 3, 5, 7];
    let res = tlargest(&v);
    println!("max is {res}");
}

fn tlargest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut p = &list[0];
    for item in list {
        if item > p {
            p = item;
        }
    }
    p
}
