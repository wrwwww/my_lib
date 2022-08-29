pub use std::fmt;

// 优先队列

pub struct PreQueue<T> where T: Ord {
    pub queue: Vec<T>,
    
}

impl<T> PreQueue<T>
where T: Ord{
    pub fn new() -> Self {
        Self {
            queue: Vec::new(),
        }
    }
    pub fn default() -> Self {
        Self {
            queue: Vec::new(),
        }
    }

    pub fn push(&mut self, element: T) {
        self.queue.push(element);
        Self::offer(self)
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.queue.len() != 0 {
            let len = self.queue.len();
            self.swap( 0, len - 1);
            let ans =self.queue.pop();
            if !self.is_empty() {
                self.offer();
            }
            ans
        } else {
            None
        }
    }
    pub fn is_empty(&self) -> bool {
        return self.queue.len() == 0;
    }
    /// 如果它是第一个元素直接插入队列
    /// 否则在插入队列后调整
    /// 找到完全二叉树的最后一个非叶子节点arr.length / 2 -1
    /// 左右节点分别是2k+1 /+2
    fn offer(&mut self) {
        if self.queue.len() != 1 {
            let index = self.queue.len() / 2 - 1;
            // 从这个节点开始自底向上检查每个节点
            for i in (0..=index).rev() {
                self.check(i);
            }
        }
    }

    fn check(&mut self, index: usize) {
        if index >= self.queue.len() {
            panic!()
        }
        let left = 2 * index + 1;
        let right = 2 * index + 2;
        let len = self.queue.len();

        // 确认该节点是否有左右节点
        if len > left && self.queue[left] > self.queue[index] {
            self.swap( index, left);
            self.check(left)
        }
        if len > right && self.queue[right] > self.queue[index] {
            self.swap(index, right);
            self.check(right)
        }
    }

    // 交换节点的值
    fn swap(&mut self, index1: usize, index2: usize) {
      self.queue.swap(index1, index2)
    }
   
}


#[test]
fn tet(){
    let mut s =PreQueue::new();
    s.push(5);
    s.push(1);
    s.push(6);
    s.push(3);
    s.push(10);
    while !s.is_empty() {
        println!("{:?}", s.pop().unwrap());
    }

}