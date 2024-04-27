#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use ylib::prelude::*;

pub mod ylib {
    pub mod prelude {
        pub use crate::ylib::index::*;
    }

    pub mod index {
        use std::{
            fmt::{self, Display, Formatter},
            ops::{Add, AddAssign, Sub, SubAssign},
            str::FromStr,
        };

        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
        pub struct OneIndex(pub usize);
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
        pub struct ZeroIndex(pub usize);

        impl From<ZeroIndex> for usize {
            fn from(value: ZeroIndex) -> Self {
                value.0
            }
        }

        impl From<OneIndex> for ZeroIndex {
            fn from(value: OneIndex) -> Self {
                ZeroIndex(value.0 - 1)
            }
        }

        impl From<ZeroIndex> for OneIndex {
            fn from(value: ZeroIndex) -> Self {
                OneIndex(value.0 + 1)
            }
        }

        impl OneIndex {
            pub fn to_zero_index(self) -> ZeroIndex {
                self.into()
            }

            pub fn to_zero_index_usize(self) -> usize {
                ZeroIndex::from(self).into()
            }
        }

        impl ZeroIndex {
            pub fn as_usize(&self) -> usize {
                self.0
            }
        }

        impl Add<usize> for ZeroIndex {
            type Output = ZeroIndex;
            fn add(self, rhs: usize) -> Self::Output {
                ZeroIndex(self.0 + rhs)
            }
        }

        impl AddAssign<usize> for ZeroIndex {
            fn add_assign(&mut self, rhs: usize) {
                self.0 += rhs;
            }
        }

        impl Add<ZeroIndex> for usize {
            type Output = ZeroIndex;
            fn add(self, rhs: ZeroIndex) -> Self::Output {
                ZeroIndex(self + rhs.0)
            }
        }

        impl Sub<usize> for ZeroIndex {
            type Output = ZeroIndex;
            fn sub(self, rhs: usize) -> Self::Output {
                ZeroIndex(self.0 - rhs)
            }
        }

        impl SubAssign<usize> for ZeroIndex {
            fn sub_assign(&mut self, rhs: usize) {
                self.0 -= rhs;
            }
        }

        impl Sub<ZeroIndex> for usize {
            type Output = ZeroIndex;
            fn sub(self, rhs: ZeroIndex) -> Self::Output {
                ZeroIndex(self - rhs.0)
            }
        }

        impl Sub<ZeroIndex> for ZeroIndex {
            type Output = usize;
            fn sub(self, rhs: ZeroIndex) -> Self::Output {
                self.0 - rhs.0
            }
        }

        impl Add<usize> for OneIndex {
            type Output = OneIndex;
            fn add(self, rhs: usize) -> Self::Output {
                OneIndex(self.0 + rhs)
            }
        }

        impl AddAssign<usize> for OneIndex {
            fn add_assign(&mut self, rhs: usize) {
                self.0 += rhs;
            }
        }

        impl Add<OneIndex> for usize {
            type Output = OneIndex;
            fn add(self, rhs: OneIndex) -> Self::Output {
                OneIndex(self + rhs.0)
            }
        }

        impl Sub<usize> for OneIndex {
            type Output = OneIndex;
            fn sub(self, rhs: usize) -> Self::Output {
                OneIndex(self.0 - rhs)
            }
        }

        impl SubAssign<usize> for OneIndex {
            fn sub_assign(&mut self, rhs: usize) {
                self.0 -= rhs;
            }
        }

        impl Sub<OneIndex> for usize {
            type Output = OneIndex;
            fn sub(self, rhs: OneIndex) -> Self::Output {
                OneIndex(self - rhs.0)
            }
        }

        impl Sub<OneIndex> for OneIndex {
            type Output = usize;
            fn sub(self, rhs: OneIndex) -> Self::Output {
                self.0 - rhs.0
            }
        }

        impl Display for OneIndex {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl Display for ZeroIndex {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl FromStr for OneIndex {
            type Err = <usize as FromStr>::Err;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(OneIndex(usize::from_str(s)?))
            }
        }

        impl FromStr for ZeroIndex {
            type Err = <usize as FromStr>::Err;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(ZeroIndex(usize::from_str(s)?))
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn one_to_zero() {
                let a = OneIndex(3);
                assert_eq!(a.to_zero_index(), ZeroIndex(2));
                assert_eq!(a.to_zero_index_usize(), 2);
            }

            #[test]
            fn zero_to_one() {
                let a = ZeroIndex(2);
                assert_eq!(OneIndex::from(a), OneIndex(3));
            }

            #[test]
            fn operators_one_index() {
                let a = OneIndex(3);
                let b = OneIndex(1);
                assert_eq!(a + 1, OneIndex(4));
                assert_eq!(1 + a, OneIndex(4));
                assert_eq!(a - 1, OneIndex(2));
                assert_eq!(a - b, 2);
            }

            #[test]
            fn assign_operators_one_index() {
                let mut a = OneIndex(3);
                a += 1;
                assert_eq!(a, OneIndex(4));
                a -= 1;
                assert_eq!(a, OneIndex(3));
            }

            #[test]
            fn operators_zero_index() {
                let a = ZeroIndex(3);
                let b = ZeroIndex(1);
                assert_eq!(a + 1, ZeroIndex(4));
                assert_eq!(1 + a, ZeroIndex(4));
                assert_eq!(a - 1, ZeroIndex(2));
                assert_eq!(a - b, 2);
            }

            #[test]
            fn assign_operators_zero_index() {
                let mut a = ZeroIndex(3);
                a += 1;
                assert_eq!(a, ZeroIndex(4));
                a -= 1;
                assert_eq!(a, ZeroIndex(3));
            }

            #[test]
            fn display_one_index() {
                let a = OneIndex(3);
                assert_eq!(a.to_string(), "3");
            }

            #[test]
            fn display_zero_index() {
                let a = ZeroIndex(3);
                assert_eq!(a.to_string(), "3");
            }
        }
    }
}

fn sort(a: &mut [i64], ops: &mut Vec<(OneIndex, OneIndex)>, index: &mut HashMap<i64, OneIndex>) {
    for i in 1..=a.len() {
        let j = *index.get(&(i as i64)).unwrap();
        if i != j.0 {
            let i = OneIndex(i);
            ops.push((i, j));
            a.swap(i.to_zero_index_usize(), j.to_zero_index_usize());
            index.insert(a[i.to_zero_index_usize()], i);
            index.insert(a[j.to_zero_index_usize()], j);
        }
    }
}

#[argio::argio]
fn main(n: usize, mut a: [i64; n]) {
    let mut ops = vec![];
    let mut index: HashMap<i64, OneIndex> = HashMap::new();
    for i in 0..n {
        index.insert(a[i], ZeroIndex(i).into());
    }
    sort(&mut a, &mut ops, &mut index);

    println!("{}", ops.len());
    for op in &ops {
        if op.0 > op.1 {
            println!("{} {}", op.1, op.0);
        } else {
            println!("{} {}", op.0, op.1);
        }
    }

    #[cfg(debug_assertions)]
    {
        dbg!(&a);
        for i in 0..a.len() {
            assert_eq!(a[i], i as i64 + 1);
        }
        assert!(ops.len() <= n - 1);
    }
}
