//! 栈和队列
//!

/**

### 队列：先进先出

 ```

#[derive(Debug)]
struct CQueue {
    queue: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CQueue {
    fn new() -> Self {
        CQueue { queue: Vec::new() }
    }

    fn append_tail(&mut self, value: i32) {
        self.queue.push(value);
    }

    fn delete_head(&mut self) -> i32 {
        if self.queue.len() == 0 {
            return -1;
        } else {
            let pop_v = self.queue[0];

               // 使用 split_off() 方法切割队列
            self.queue = self.queue.split_off(1);

            pop_v
        }
    }
}

```
*/

pub fn queue() {}

/**

### 栈：只能从栈顶弹出

```

/// 一个经典问题：pop、top、min、push操作皆为 O（1）复杂度算法

struct MinStack {
    stack_a: Vec<i32>,
    stack_b: Vec<i32>,
}

impl MinStack {
    /** initialize your data structure here. */
    fn new() -> Self {
        MinStack {
            stack_a: Vec::new(),
            stack_b: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        let l = self.stack_a.len();

        if self.stack_a.len() == 0 {
            self.stack_a.push(x);
            self.stack_b.push(x);
        } else {
            if self.stack_b[l - 1] < x {
                self.stack_a.push(x);
                self.stack_b.push(self.stack_b[l - 1]);
            } else {
                self.stack_a.push(x);
                self.stack_b.push(x);
            }
        }
    }

    fn pop(&mut self) {
        if self.stack_a.len() == 0 {
            println!("stack is empty");
        }
        let l = self.stack_a.len();
        self.stack_b.split_off(l - 1);
        self.stack_a.split_off(l - 1);
    }

    fn top(&self) -> i32 {
        if self.stack_a.len() == 0 {
            println!("stack is empty");
        }

        let l = self.stack_a.len();

        self.stack_a[l - 1]
    }

    fn min(&self) -> i32 {
        let l = self.stack_a.len();

        self.stack_b[l - 1]
    }
}

```
*/

pub fn stack() {}
