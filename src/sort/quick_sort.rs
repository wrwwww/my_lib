pub use super::is_sorted;
// 快速排序
pub fn quick_sort<T: Ord >(nums: &mut [T]) {
    // 没有办法保证数组长度不为0 ,所以使用更安全的isize
    _quick_sort(nums, 0, nums.len() as isize-1);
}

fn _quick_sort<T: Ord >(nums: &mut [T], start: isize, end: isize) {
     // 判断是否可以排序
     // 当end=start 只有一个数字不用操作
     // end<start 更不用操作
     if start < end {
        // 得到基准数下标
        let temp=sort(nums, start,end);
        // 对基准数前基准数后进行排序
        _quick_sort(nums, start, temp - 1);
        _quick_sort(nums, temp + 1, end);
    }
    
}
fn sort<T: Ord >(nums: &mut [T], start: isize, end: isize)->isize {
   
    let mut left = start;
    let mut right = end;
    // 基准的数的索引
    let temp = left as usize;

    loop {
       while nums[left as usize]<nums[temp] &&left<right {
           left+=1;
       }
       while nums[right as usize]>nums[temp] {
           right-=1;
       }
       if left>=right {
           break;
       }
       // 头指针找到比基准数大的数
       // 后指针找到比基准数小的数
       // 交换两个数
       nums.swap(left as usize, right as usize);
    }
    // 达到跳出循环的条件
    // right和left相撞
    // 这个位置的数字一点<=基准数
    nums.swap(temp, right as usize);
    right
}
#[test]
fn test() {
    let mut nums = [9, 5, 10, 7];
    quick_sort(&mut nums);
    assert!(is_sorted(&nums));
}
