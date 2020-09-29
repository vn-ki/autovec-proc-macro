use autovec::autovec;
struct Point(i64, i64);

#[autovec]
fn fn_4(Point(x, y): Point) -> i64 {
   x * y
}

fn main() {}
