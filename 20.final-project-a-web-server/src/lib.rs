use std::thread;
use std::sync::{mpsc, Arc, Mutex};

pub struct ThreadPool {
    // threads: Vec<thread::JoinHandle<()>>,
    workers: Vec<Worker>,
    // 充当发送端的开始
    sender: mpsc::Sender<Job>,
}

// struct Job;
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    // 若不适用 assert! 宏, 也可以使用如下签名:
    // pub fn new(size: useize) -> Result<ThreadPool, PoolCreationError>
    pub fn new(size: usize) -> ThreadPool {
        // 限制线程必须 > 0
        assert!(size > 0);

        // `with_capacity` 为 vector 预先分配空间
        // 比 `Vec::new` 要更有效率, 不会随着插入元素重新改变大小
        // let mut threads = Vec::with_capacity(size);

        // 增加发送接收
        let (sender, receiver) = mpsc::channel();

        // Arc 使得多个 worker 拥有接收端
        // Mutex 确保一次只有一个 worker 能从接收端得到任务
        let receiver = Arc::new(Mutex::new(receiver));

        // 修改为 worker
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // 新建线程并存储

            // 新建 Worker
            // 尝试将接收者传递给多个 `Worker` 实例, 不行
            // 仅可以多 `生产者`, 单 `消费者`
            // 从通道队列中取出任务涉及到修改 `receiver`, 所以这些线程需要一个能安全的共享和修改接收者的方式
            // 否则可能导致竞争状态
            // workers.push(Worker::new(id, receiver));
            // ^^^^^^^^ value moved here, in previous iteration of loop

            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        
        ThreadPool {
            // threads
            workers,
            sender
        }
    }

    pub fn execute<F>(&self, f: F)
    where
    // 这里的 `FnOnce()` 代表一个没有参数也没有返回值的闭包, 返回值类型可以从签名中省略,
    // 不过即使没有参数也需要括号
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        // 在通道中发出任务
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}

impl Worker {
    // 接收: 接收端
    // fn new(id: usize, receiver: mpsc::Receiver<Job>) -> Worker {
    //     // 传递的闭包没有做任何工作
    //     // let thread = thread::spawn(|| {});

    //     // 接受接收端
    //     let thread = thread::spawn(|| {
    //         receiver;
    //     });

    //     Worker {
    //         id,
    //         thread
    //     }
    // }

    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            // 循环请求任务
            loop {
                // 首先调用 `lock` 获取互斥器
                // 如果互斥器处于一种叫做 *被污染(poisoned)* 的状态时获取锁可能会失效
                // 可能发生于其他线程在持有锁时 panic 了且没有释放锁

                // 接着调用 `recv` 从通道接收 `Job`
                // 调用 `recv` 会阻塞当前线程
                // Mutex<T> 确保一次只有一个 `Worker` 线程尝试请求任务
                let job = receiver.lock().unwrap().recv().unwrap();

                println!("Worker {} got a job; executing.", id);

                job();
            }
        });

        Worker {
            id,
            thread
        }
    }

    // 尝试使用 `while let`
    // fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
    //     let thread = thread::spawn(move || {
    //         // 可以编译运行, 但是一个慢请求仍然会导致其他请求等待执行.
    //         // `Mutex` 结构体没有公有 `unlock` 方法,
    //         // 因为锁的所有权依赖 `lock` 方法你会的 `LockResult<MutexGuard<T>>` 中 `MutexGurad<T>` 的生命周期
    //         // 这允许借用检查器在编译时确保绝不会在没有持有锁的情况下访问由 `Mutex` 守护的资源,
    //         // 如果没有认真思考 `MutexGuard<T>` 的生命周期的话, 也有可能会导致比预期更久的持有锁.
    //         // 因为 `while` 表达式中的值在整个块一只处于作用域中
    //         // `job()` 的调用过程中仍然持有锁, 这意味着其他 worker 不能接受任务
    //         // loop 内 lock 方法返回的 `MutexGuard` 在 `let job` 语句结束之后立刻就被丢弃了
    //         // 这确保了 `recv` 调用过程中持有锁
    //         while let Ok(job) = receiver.lock().unwrap().recv() {
    //             println!("Worker {} got a job; exeuting.", id);

    //             job();
    //         }
    //     });

    //     Worker {
    //         id,
    //         thread
    //     }
    // }
}
