use autovec::autovec;

struct Point(i64, i64);

#[autovec]
fn fn_4([a, b, c]: [i64; 3]) -> i64 {
    1
}

fn main() {}
