use crate::orst::sort_lib::Sorter;

pub struct InsertionSort;

impl Sorter for InsertionSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
        for unsorted in 1..slice.len() {
            let mut i = unsorted;
            while i > 0 && slice[i] < slice[i - 1] {
                slice.swap(i, i - 1);
                i -= 1;
            } //可以使用二分查找  binary_search
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn qort() {
        let mut things = vec![
            58, 12, 94, 3, 47, 81, 25, 66, 19, 38, 72, 5, 89, 41, 14, 97, 30, 52, 63, 21,
        ];
        InsertionSort::sort(&mut things);
        assert_eq!(
            things,
            &[
                3, 5, 12, 14, 19, 21, 25, 30, 38, 41, 47, 52, 58, 63, 66, 72, 81, 89, 94, 97
            ]
        )
    }
}
