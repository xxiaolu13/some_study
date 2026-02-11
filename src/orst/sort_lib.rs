pub trait Sorter {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord;
}

#[cfg(test)]
mod tests {
    use super::*;
    struct StdSorter;
    impl Sorter for StdSorter {
        fn sort<T>(slice: &mut [T])
        where
            T: Ord,
        {
            slice.sort();
        }
    }

    #[test]
    fn std_sort() {
        let mut things = vec![4, 3, 2, 1];
        StdSorter::sort(&mut things);
        assert_eq!(things, &[1, 2, 3, 4])
    }
}
