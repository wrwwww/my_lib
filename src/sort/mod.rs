mod bubble_sort;
mod merge_sort;
mod quick_sort;

pub use bubble_sort::bubble_sort;
pub use merge_sort::merge_sort;
pub use quick_sort::quick_sort;

pub fn is_sorted<T>(arr: &[T]) -> bool
where
    T: Ord,
{
    if arr.is_empty() {
        return true;
    }
    let mut pre = &arr[0];
    // skip表示迭代器跳过的数字个数
    // [1,2,3,4] skip(1) 后为 2,3,4
    // 因为从第二个数和前一个数比
    for i in arr.iter().skip(1) {
        if pre > i {
            return false;
        }
        pre = i;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let res = vec![10, 8, 4, 3, 1, 9, 2, 7, 5, 6];
        let res2 = vec!["a", "bb", "d", "cc"];
        _sort(bubble_sort, &mut res.clone());
        _sort(bubble_sort, &mut res2.clone());
        _sort(quick_sort, &mut res.clone());
        _sort(quick_sort, &mut res2.clone());
        _sort(merge_sort, &mut res2.clone());
        _sort(merge_sort, &mut res.clone());
    }

    fn _sort<T: Ord>(sort: fn(&mut [T]), nums: &mut [T]) {
        sort(nums);
        assert!(is_sorted(nums));
    }

    #[test]
    fn bubble_sort_test() {
        let mut nums = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        bubble_sort(&mut nums);
        assert!(is_sorted(&nums))
    }
    #[test]
    fn quick_sort_test() {
        let mut nums = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        quick_sort(&mut nums);
        assert!(is_sorted(&nums))
    }
    #[test]
    fn merge_sort_test() {
        let mut nums = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        merge_sort(&mut nums);
        assert!(is_sorted(&nums))
    }
}
