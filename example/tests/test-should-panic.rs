/// Should panic if the length of arguments vectors are not all same
use autovec::autovec;
#[autovec]
fn fn_3<X: Into<usize>, Y: Into<usize>>( a: X, b: Y, c: String) -> usize {
    return a.into() + b.into();
}

#[test]
#[should_panic]
fn test() {
    let a: Vec<usize> = vec![1, 2, 3, 0];
    let b: Vec<usize> = vec![3, 2, 0];
    let c: Vec<String> = vec!["a".into(), "b".into(), "c".into(), "d".into()];
    assert_eq!(vec![4, 4, 3, 0], fn_3(a, b, c));
}
