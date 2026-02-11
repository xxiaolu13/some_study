use crate::orst::sort_lib::Sorter;
// gnuplot
pub struct QuickSort;

fn quick_sort<T>(slice: &mut [T])
where
    T: Ord,
{
    match slice.len() {
        1 | 0 => return,
        2 => {
            if slice[0] > slice[1] {
                slice.swap(0, 1);
            }
            return;
        }
        _ => {}
    }
    let (pivot, rest) = slice.split_first_mut().expect("slice is empty");
    let mut left = 0;
    let mut right = rest.len() - 1;

    while right != 0 && left <= right {
        if &rest[left] <= pivot {
            left += 1;
        } else if &rest[right] > pivot {
            right -= 1;
        } else {
            rest.swap(left, right);
            left += 1;
            right -= 1;
        }
        println!("l:{}  r:{}", left, right);
    }
    slice.swap(0, left);
    let (l_slice, r_slice) = slice.split_at_mut(left);
    quick_sort(l_slice);
    quick_sort(&mut r_slice[1..]);
}

impl Sorter for QuickSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
        quick_sort(slice);
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
        QuickSort::sort(&mut things);
        assert_eq!(
            things,
            &[
                3, 5, 12, 14, 19, 21, 25, 30, 38, 41, 47, 52, 58, 63, 66, 72, 81, 89, 94, 97
            ]
        )
    }
}
