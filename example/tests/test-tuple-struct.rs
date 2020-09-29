use autovec::autovec;
struct Point(i64, i64);

#[autovec]
fn fn_4(Point(x, y): Point) -> i64 {
   x * y
}

#[test]
fn test_tuple_struct() {
    let a: Vec<_> = vec![
        Point(1, 2),
        Point(3, 4),
    ];
    assert_eq!(fn_4(a), vec![2, 12]);
}
