#[allow(unused_imports)]
use std::collections::*;

use ac_library::ModInt998244353 as mint;

#[argio::argio]
fn main(n: usize, d: [i64; n]) -> u32 {
    if d[0] != 0 {
        return 0;
    }

    let mut c = HashMap::new();

    for i in 1..n {
        *c.entry(d[i]).or_insert(mint::from(0)) += 1;
    }

    if c.get(&0).is_some() {
        return 0;
    }

    let mut ans = mint::from(1);
    for i in 1..n {
        ans *= c.get(&(d[i] - 1)).unwrap_or(&mint::from(1));
    }

    ans.val()
}
