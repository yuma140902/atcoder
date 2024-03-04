mod table_display {
    pub struct TableDisplay<'a, T>(&'a Vec<Vec<T>>);
    impl<T: std::fmt::Display> std::fmt::Display for TableDisplay<'_, T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for x in 0..self.0.len() {
                for y in 0..self.0[x].len() {
                    write!(f, "{} ", self.0[x][y])?;
                }
                writeln!(f)?;
            }
            Ok(())
        }
    }
}
