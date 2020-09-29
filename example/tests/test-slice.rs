use autovec::autovec;

struct Point(i64, i64);

#[autovec]
fn fn_4([a, b, c]: [i64; 3]) -> i64 {
    a*b*c
}

#[test]
fn test_slice() {
    let a: Vec<_> = vec![
        [1, 2, 3],
        [3, 4, 5]
    ];
    assert_eq!(fn_4(a), vec![6, 60]);
}
