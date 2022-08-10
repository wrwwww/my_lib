// 归并排序
// 一直对半分,直到不能再分,然后返回
// 对返回会来的进行有序合并
pub fn merge_sort<T: Ord + Copy>(arr: &mut [T]) {
    _merge_sort(arr, 0, arr.len() as isize)
}
fn _merge_sort<T: Ord + Copy>(arr: &mut [T], start: isize, end: isize) {
    if start >= end - 1 {
        return;
    }
    let mid = start + (end - start) / 2;
    _merge_sort(arr, start, mid);
    _merge_sort(arr, mid, end);
    merge(arr, start, mid, end);
}
fn merge<T: Ord + Copy>(arr: &mut [T], start: isize, mid: isize, end: isize) {
    // 合并两个有序子串
    let mut left = 0;
    let mut right = 0;
    let arr1 = arr[start as usize..mid as usize].to_vec();
    let arr2 = arr[mid as usize..end as usize].to_vec();
    // 合并两个有序数组到原数组
    for i in start..end{
        // 放左边的数组的情况
        // 1. 当右边的数组都放进去
        // 2. 左边的数组没放完并且左边现在的数大于右边的数字
        if right >= arr2.len()||(left < arr1.len()&&arr1[left] < arr2[right]){
            arr[i as usize] = arr1[left];
            left += 1;
        } else {
            arr[i as usize] = arr2[right];
            right += 1;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic() {
        let mut res = vec![10, 8, 4, 3, 1, 9, 2, 7, 5, 6];
        merge_sort(&mut res);
        assert_eq!(res, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn basic_string() {
        let mut res = vec!["a", "bb", "d", "cc"];
        merge_sort(&mut res);
        assert_eq!(res, vec!["a", "bb", "cc", "d"]);
    }

    #[test]
    fn empty() {
        let mut res = Vec::<u8>::new();
        merge_sort(&mut res);
        assert_eq!(res, vec![]);
    }

    #[test]
    fn one_element() {
        let mut res = vec![1];
        merge_sort(&mut res);
        assert_eq!(res, vec![1]);
    }

    #[test]
    fn pre_sorted() {
        let mut res = vec![1, 2, 3, 4];
        merge_sort(&mut res);
        assert_eq!(res, vec![1, 2, 3, 4]);
    }

    #[test]
    fn reverse_sorted() {
        let mut res = vec![4, 3, 2, 1];
        merge_sort(&mut res);
        assert_eq!(res, vec![1, 2, 3, 4]);
    }
}
