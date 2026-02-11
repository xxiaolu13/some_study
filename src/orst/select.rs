use crate::orst::sort_lib::Sorter;

pub struct SelectSort;

impl Sorter for SelectSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
        for i in 0..(slice.len() - 1) {
            // let mut flag = i;
            // for j in i + 1..slice.len() {
            //     if slice[j] < slice[i] {
            //         flag = j;
            //     }
            // }
            let min_index = slice[i..]
                .iter()
                .enumerate()
                .min_by_key(|&(_, value)| value)
                .map(|(idx, _)| i + idx) // 加上偏移量 i，得到在原 slice 中的索引
                .unwrap();
            // let flag = slice.iter().min();
            slice.swap(min_index, i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn select_sort() {
        let mut things = vec![4, 3, 2, 1];
        SelectSort::sort(&mut things);
        assert_eq!(things, &[1, 2, 3, 4])
    }
}
