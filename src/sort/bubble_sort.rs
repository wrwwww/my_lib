pub fn bubble_sort<T: Ord>(nums: &mut [T]) {
    if nums.is_empty() {
        return;
    }
    let mut len = nums.len();
    // 标志位
    // false 为有改变 就去循环
    // TRUE 说明没有循环 说明序列已经排好序啦
    let mut flag = false;
    while !flag {
        flag = true;
        // 当len=1的时候就是最后一次遍历
        // len如果直接减去一会再最后有一个额外的减一的操作
        // 使usize溢出
        for i in 0..len - 1 {
            if nums[i] > nums[i + 1] {
                nums.swap(i, i + 1);
                flag = false;
            }
        }
        len -= 1;
    }
}

#[test]
fn test() {
    let mut ve1 = vec![6, 5, 4, 3, 2, 1];
    bubble_sort(&mut ve1);
    print!("{:?}", ve1);
}

