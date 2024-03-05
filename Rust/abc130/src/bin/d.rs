#[allow(unused_imports)]
use std::collections::*;

#[argio::argio]
fn main(n: usize, k: u64, a: [u64; n]) -> usize {
    let mut count = 0;

    let mut right = 0;
    let mut sum = 0;
    for left in 0..n {
        /* [left, right)の総和がk以上になる最小のrightを求める */
        while right < n && sum < k {
            sum += a[right];
            right += 1;
        }

        if sum < k {
            /* [left, right)の総和がk以上になるrightは存在しない */
            break;
        }
        count += n - (right - 1);

        if left == right {
            /* leftがrightに追いついたらrightも一緒に増やす */
            right += 1;
        } else {
            /* leftがインクリメントされるのでsumを減らす */
            sum -= a[left];
        }
    }

    count
}
