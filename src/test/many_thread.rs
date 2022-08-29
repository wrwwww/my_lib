use std::sync::{Arc, Mutex};

#[test]
fn test1() {
    // 简单的闭包
    let sum=|a:i32,b:i32|->i32 {
       return a+b;
    };
    dbg!(sum(1,9));
}

///
/// 多个线程打印字符
#[test]
fn test2() {
    let mut handles=vec![];
    for _ in 0..5{
        // 多线程
        handles.push(std::thread::spawn(||{
            let id=std::thread::current().id();
            for i in 0..=10{
                println!("线程id={:?}\n字符{}",id,i);
            }
        }));
        
    }
    // 等待其他线程
    for handle in handles {
        handle.join().unwrap();
    }
}
/// 多个线程对一个数进行操作
#[test]
fn test3() {
    let mut handles=vec![];
    let num=20;
    let num=arc_new(num);
    for _ in 0..3 {
        let num1=new_clone(&num);
        handles.push(std::thread::spawn(move||{
            // let mut s=;
            let id=std::thread::current().id();
            while *num1.lock().unwrap()>0 {
                *num1.lock().unwrap()-=1;
                println!("线程id={:?}\n字符地址:{}",id,*num1.lock().unwrap());
            }
        }))
    }   
     // 等待其他线程
     for handle in handles {
        handle.join().unwrap();
    }
}
fn arc_new<T>(v:T)->Arc<Mutex<T>>{
    Arc::new(Mutex::new(v))
}

fn new_clone<T>(v: &Arc<Mutex<T>>) -> Arc<Mutex<T>> { 
    Arc::clone(v)
}