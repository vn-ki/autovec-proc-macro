use autovec::autovec;
struct Location {
   x: i64,
   y: i64,
}

#[autovec]
fn fn_4((Location { x, .. }, Location { y, .. }): (Location, Location)) -> i64 {
   x * y
}


#[test]
fn test_tuple() {
    let a: Vec<_> = vec![
        (Location {x: 1, y: 2}, Location {x: 3, y: 4}),
        (Location {x: 1, y: 2}, Location {x: 3, y: 4}),
    ];
    assert_eq!(fn_4(a), vec![4, 4]);
}
