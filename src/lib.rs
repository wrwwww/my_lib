pub mod dp;
pub mod graph;
pub mod recall;
pub mod sort;
pub mod utils;
pub mod arrays;
pub mod test;
#[cfg(test)]
mod tests {

    #[derive(Debug)]
    struct Data {
        num: usize,
    }
    impl Drop for Data {
        fn drop(&mut self) {
            println!("销毁")
        }
    }

    #[test]
    fn it_works() {
        let a = Data { num: 10 };
        let t = &a as *const Data;
        println!("1.{:?},{:?}", a, t);
        ss(a);
        // println!("2.{:?},{:?}",a,t);
        unsafe {
            println!("{:p}, {:?}", t, *t);
            let a = &*t;
            println!("{}", a.num)
        }
    }
    fn ss(_: Data) {}
        use std::{cell::RefCell, collections::HashMap, mem::size_of_val, rc::Rc};

    #[test]
    fn t2() {
        let mut a = 100;
        let b: *mut i32 = &mut a;
        unsafe {
            *b += *b;
            // 指针可以转为usize然后运算
            println!("b的指针地址{}", b as usize);
            println!("a所占的字节{}", size_of_val(&a));
            println!("{:p}", size_of_val(&a) as *mut i32);
            println!("{:?}", (b as usize) + size_of_val(&a));
        }
        println!("{}", a);
    }
    #[test]
    fn t1() {
        use std::mem::size_of;
        use std::mem::size_of_val;
        let a: [u8; 10] = [2, 3, 4, 5, 6, 7, 8, 9, 0, 1];
        let b = &a as *const [u8];
        // b的内容为a 的地址
        println!("b的地址{:p}", &b);
        println!("b指向的地址{:p}", b);
        println!("b内存大小{}", size_of_val(&b));
        println!("b内存大小{}", size_of::<*const [u8]>());
        unsafe {
            println!("a内存大小{}", size_of_val(&(*b)));
        }

        println!("a的地址{:p}", &a);
        println!("a的内容{:?}", &a);
        println!("a内存大小{}", size_of_val(&a));
        println!("a内存大小{}", size_of::<[u8; 10]>());
        println!("");

        unsafe {
            let mut addr = &a[0] as *const u8;
            println!("address:{:p}", addr);
            println!("内容:{:?}", *addr); // 2
            let s = size_of_val(&a[0]);
            // std::mem::
            addr = (addr as usize + s) as *const u8;
            println!("address:{:p}", addr);
            println!("内容:{:?}", *addr); // 3
        }
    }
    #[test]
    fn test2() {
        let mut map = HashMap::<i32, i32>::new();
        map.insert(1, 1);
        map.insert(2, 2);
        let p = &mut map as *mut HashMap<i32, i32>;
        // *p拿到map
        unsafe {
            let p1 = &mut map;
            (*p).insert(3, 3);
            p1.insert(4, 4);
        }
        println!("{:?}", map);
    }
    #[test]
    fn test3() {
        let mut map = HashMap::<i32, i32>::new();
        map.insert(1, 1);
        map.insert(2, 2);
        // 内部可变
        let map2 = map.clone();
        std::mem::drop(map);
        let s = Rc::new(RefCell::new(map2));
        let s1=Rc::clone(&s);
        println!("{}",Rc::strong_count(&s));
        unsafe {
            // 生指针
            let s1=(*s).as_ptr();
            (*s1).insert(4, 4);            
        }
        let _ss=(*s1).borrow_mut().insert(4, 10);
        println!("map={:?}",(*s1).borrow());
    }
    #[derive(Debug)]
    struct  List {
        head:Rc<RefCell<Node>>,
        end:Rc<RefCell<Node>>,
        len:usize,
    }
    impl List {
        fn new()->Self{
            let head=Rc::new(RefCell::new(Node::new(0)));
            let end=Rc::clone(&head);
            let len=0;
            Self { head,end,len}
        }
        fn push(&mut self,value:i32){
            (*self.end).borrow_mut().set_next(value);
            let next=Rc::clone(
                (*self.end)
                .borrow_mut()
                .next.as_ref()
                .unwrap());
            self.end=next;
            self.len+=1;
        }
        fn len(&self)->usize{
            self.len
        }
        fn pop(&mut self)->Option<i32>{
            if self.len==0 {
                return None;
            }
            let len=self.len()-1;
            let mut i=0;
            let mut temp=Rc::clone(&self.head);
            // 找到倒数第二个node
            while i!=len {
                i+=1;
                let t2=Rc::clone((*temp).borrow_mut().next.as_ref().unwrap());
                temp=t2;
            }
            // if let Some(x)= (*temp)
            //     .borrow_mut()
            //     .next.as_ref(){
            //     let val=x.borrow().get_val();
            // };
            let val=((*temp)
                .borrow_mut()
                .next.as_ref()
                .unwrap())
                .borrow()
                .get_val();
            (*temp).borrow_mut().next=None;
            self.end=Rc::clone(&temp);
            self.len-=1;
            val
        }
    }
    #[derive(Debug)]
    struct Node{
        val:i32,
        next:Option<Rc<RefCell<Node>>>,
    }
    impl Node {
        fn new(value:i32)->Self{
            Self { val: value, next: None }
        }
        fn set_next(&mut self,value:i32){
            self.next=Some(Rc::new(RefCell::new(Node::new(value))));
        }
        fn get_val(&self)->Option<i32>{
            Some(self.val)
        }
    }
    #[test]
    fn test4(){
        let mut s=List::new();    
        s.push(1);
        s.push(2);
        s.push(3);
        s.push(4);
        // println!("{:#?}",s);
        println!("{}",s.pop().unwrap());
        println!("{}",s.pop().unwrap());
        println!("{}",s.pop().unwrap());
        println!("{}",s.pop().unwrap());
        println!("{:?}",s.pop());
        
    }
    #[test]
    fn test5(){
        let a=10;
        dbg!(
            a
        );
    }
    ///
    /// todo!() 可以在你没写函数内容及返回值的时候使用
    /// 这样不会报错
    #[allow(dead_code)]
    fn test_fun()->i32{
        todo!()
    }
}
