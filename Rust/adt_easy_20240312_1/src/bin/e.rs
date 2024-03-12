#[allow(unused_imports)]
use std::collections::*;

const NONE: char = '0';
const FOOD: char = '1';
const EVENT: char = '2';

#[argio::argio]
fn main(n: usize, m: i32, s: String) -> i32 {
    let s = s.chars().collect::<Vec<_>>();
    let mut muji_bought = 0;
    let mut logo_bought = 0;

    let mut muji_all = m;
    let mut logo_all = 0;

    let mut muji_clean = muji_all;
    let mut logo_clean = logo_all;
    for c in s {
        if c == NONE {
            muji_clean = muji_all;
            logo_clean = logo_all;
        } else if c == FOOD {
            if muji_clean > 0 {
                muji_clean -= 1;
            } else if logo_clean > 0 {
                logo_clean -= 1;
            } else {
                logo_bought += 1;
                logo_all += 1;
            }
        } else {
            if logo_clean > 0 {
                logo_clean -= 1;
            } else {
                logo_bought += 1;
                logo_all += 1;
            }
        }
    }

    muji_bought + logo_bought
}
