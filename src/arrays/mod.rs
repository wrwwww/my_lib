pub fn bin_search(arr:Vec<i32>,target:i32)->isize{
    let mut left=0;
    let mut right=arr.len();
    while left<right {
        let mid=left+(right-left)/2;
        if arr[mid]==target {
            return mid as isize;
        }
        if arr[mid]<target {
            left=mid+1;
        }else {
            right=mid;
        }
    }
   return -1;
}
// 0 2
// 返回插入后的下标
pub fn insert_index(arr:Vec<i32>,target:i32)->usize{
    let mut left=0;
    let mut right=arr.len();
    while left<right {
        let mid=left+(right-left)/2;
        if arr[mid]==target {
            return mid;
        }
        if arr[mid]<target {
            left=mid+1;
        }else {
            right=mid;
        }
    }
   return left;

}
#[test]
fn test(){
    let s=[1,2,3,4,5,6];
    println!("{:?}",bin_search(s.to_vec(), 1));
    println!("{:?}",bin_search(s.to_vec(), 2));
    println!("{:?}",bin_search(s.to_vec(), 3));
    println!("{:?}",bin_search(s.to_vec(), 4));
    println!("{:?}",bin_search(s.to_vec(), 5));
    println!("{:?}",bin_search(s.to_vec(), 6));
}
#[test]
fn test1(){
    let s=[1,2,3];
    // [1,2,3]; left=0 right=3 mid=1 target=1
    // [1,2]; left=0 right=1 mid=0
    println!("{:?}",bin_search(s.to_vec(), 1));

}
#[test]
fn test2(){
    let s=[];
    println!("{:?}",bin_search(s.to_vec(), 1));
}