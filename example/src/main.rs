use autovec::autovec;
#[autovec]
fn fn_3<X: Into<usize>, Y: Into<usize>>( a: X, b: Y, c: String) -> usize {
    return 1;
}

fn main() {
    let a: Vec<usize> = vec![1, 2, 3, 0];
    let b: Vec<usize> = vec![3, 2, 0, 0];
    let c: Vec<String> = vec!["a".into(), "b".into(), "c".into(), "d".into()];
    println!("{:?}", fn_3(a, b, c)); // [4), 4), 0, 0]
}
