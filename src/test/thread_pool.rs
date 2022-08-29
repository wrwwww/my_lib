use std::{sync::{mpsc, Arc, Mutex}, thread};
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}
type Job = Box<dyn FnOnce() + Send + 'static>;
pub enum Message{
    NewJob(Job),
    Terminate,
}
impl ThreadPool {
    pub fn new(num: usize) -> Self {
        assert!(num> 0);
        let mut workers: Vec<Worker> = Vec::with_capacity(num);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        for i in 0..workers.len() {
            workers.push(Worker::new(i, Arc::clone(&receiver)));
        }
        Self { workers, sender }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // 发送到空闲的线程执行
        let job = Box::new(f);
        if let Err(msg)=self.sender.send(Message::NewJob(job)){
            println!("{:?}",msg);
        }
        // self.sender.send( Message::NewJob(Box::new(f))).unwrap();
    }
}
impl Drop for ThreadPool {
    fn drop(&mut self) {
         // 发送停止命令
         for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            worker.handle.take().unwrap().join().unwrap();
        }
    }
}
struct Worker {
    id: usize,
    handle: Option<thread::JoinHandle<()>>,
}
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
        let handle = thread::spawn(move || loop{
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job)=>{
                    job();
                },
                Message::Terminate=>{
                    println!("线程退出");
                    break;
                }    
            }
        });
        Self { id, handle:Some(handle) }
    }
}


#[test]
fn test1(){
    let pool=ThreadPool::new(4);
    for _ in 0..4 {
        pool.execute(||{
            // println!("ii");
            let a=10;
        });      
    }
}