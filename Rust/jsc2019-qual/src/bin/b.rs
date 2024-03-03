use ac_library::ModInt1000000007 as ModInt;

#[argio::argio]
fn main(n: usize, k: u64, a: [u64; n]) -> u32 {
    let mut greater = vec![0; n];
    let mut greater_right = vec![0; n];

    for i in 0..a.len() {
        greater[i] = a.iter().filter(|aj| a[i] > **aj).count();
        greater_right[i] = a.iter().skip(i).filter(|aj| a[i] > **aj).count();
    }

    let mut inversions = vec![ModInt::new(0); n];
    for i in 0..a.len() {
        inversions[i] =
            ModInt::new(greater[i]) * (ModInt::new(k) - ModInt::new(1)) * ModInt::new(k) / 2
                + ModInt::new(greater_right[i]) * ModInt::new(k);
    }

    inversions.iter().sum::<ModInt>().val()
}
