use crate::orst::sort_lib::Sorter;

pub struct BubbleSort;

impl Sorter for BubbleSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
        let mut swapped = true;
        while swapped {
            swapped = false;
            for j in 0..slice.len() {
                if j == slice.len() - 1 {
                    continue;
                }
                if slice[j] > slice[j + 1] {
                    slice.swap(j, j + 1);
                    swapped = true;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bubble_sort() {
        let mut things = vec![4, 3, 2, 1];
        BubbleSort::sort(&mut things);
        assert_eq!(things, &[1, 2, 3, 4])
    }
}
