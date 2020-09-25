use autovec::autovec;
#[autovec]
fn add(a: Option<usize>, b: Option<usize>, c: Option<usize>) -> Option<usize> {
    if let Some(a) = a {
        if let Some(b) = b {
            if let Some(c) = c {
                return Some(a + b + c);
            }
        }
    }
    return None;
}

fn main() {
}
