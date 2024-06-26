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

use petgraph::{algo::TarjanScc, graph::Graph, Undirected};

#[argio::argio]
fn main(n: usize, m: usize, ab: [(OneIndex, OneIndex); m]) -> i64 {
    let mut edges = Vec::with_capacity(m * 2);
    for &(a, b) in &ab {
        edges.push((a, b));
        edges.push((b, a));
    }

    let graph: Graph<(), (), Undirected, _> =
        Graph::from_edges(edges.into_iter().map(|(i, j)| (i.0, j.0)));

    let mut tarjan_scc = TarjanScc::new();
    let mut result: Vec<HashSet<_>> = Vec::new();
    tarjan_scc.run(&graph, |scc| {
        result.push(scc.iter().map(|node_idx| node_idx.index()).collect())
    });

    let mut count = 0;

    for node_set in &result {
        let n = node_set.len() as i64;
        count += (n - 1) * n / 2;
    }

    count -= m as i64;

    count
}
