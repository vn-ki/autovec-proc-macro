use autovec::autovec;
struct Location {
   x: i64,
   y: i64,
}

#[autovec]
fn fn_4((Location { x, .. }, Location { y, .. }): (Location, Location)) -> i64 {
   x * y
}

fn main() {
    let a: Vec<_> = vec![
        (Location {x: 1, y: 2}, Location {x: 3, y: 4}),
        (Location {x: 1, y: 2}, Location {x: 3, y: 4}),
    ];
    println!("{:?}", fn_4(a)); // [4), 4), 0, 0]
}
