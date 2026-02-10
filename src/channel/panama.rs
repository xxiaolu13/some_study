// 这个mod是关于channel的
// 1. 这里的同步channel基于mutex实现
// 2. condvar等待机制，当获取到锁，但是channel里面是空的，会进入等待，wait 方法会自动释放锁并将线程挂起（阻塞）。当被 notify 唤醒时，wait 会在返回前重新获取锁
// 3. model实现，shared和inner，shared中包括锁和convar，convar见2，用于实现等待机制，mutex里面为inner，inner里面分别时vecdeque和sender
// vecdeque是一个环状的vec，其在头和尾部加上了一个类似指示的东西
// sender用于解决，当这个channel的tx都没有了，rx还在继续等待接收，逻辑为 当drop tx时候，会将计数减去1，而recv会返回一个option，当tx计数都是1，rx会返回none，见test2
// 4. clone的细节，给包裹了arc的实现clone，arc内部的东西可以不实现clone，但是如果用derive默认的clone他会要求 arc内部的T: Clone
// 5. sender 的 drop 最后如果变成0会通知receiver，如果receiver在等待的过程中，sender变成了0，会直接被叫醒，然后进入下一次loop就退出了

use std::{
    collections::VecDeque,
    sync::{Arc, Condvar, Mutex},
};

pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}

pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
}

struct Inner<T> {
    queue: VecDeque<T>,
    senders: usize,
}

struct Shared<T> {
    inner: Mutex<Inner<T>>,
    available: Condvar,
}

// 手动实现clone，如果用derive默认实现clone，他会要求T也要实现clone，但是使用arc了，他不需要T实现clone，所以手动实现
impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders += 1;
        drop(inner);
        Sender {
            shared: Arc::clone(&self.shared),
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders -= 1;
        let was_last = inner.senders == 0;
        drop(inner);
        // 如果是最后一个 则通知receiver
        if was_last {
            self.shared.available.notify_one();
        }
    }
}

impl<T> Sender<T> {
    pub fn send(&mut self, t: T) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.queue.push_back(t);
        drop(inner);
        self.shared.available.notify_one();
    }
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> Option<T> {
        let mut inner = self.shared.inner.lock().unwrap();
        loop {
            match inner.queue.pop_front() {
                Some(t) => return Some(t),
                None if inner.senders == 0 => return None,
                None => {
                    inner = self.shared.available.wait(inner).unwrap();
                    // wait 会：
                    //    a) 释放 queue 持有的锁（其他线程现在可以访问队列了）
                    //    b) 阻塞当前线程
                    //    c) 被 notify_one() 唤醒后
                    //    d) 重新获取锁
                    //    e) 返回新的 MutexGuard 赋值给 queue
                }
            }
        }
    }
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Inner {
        queue: VecDeque::default(),
        senders: 1,
    };
    let shared = Shared {
        inner: Mutex::new(inner),
        available: Condvar::new(),
    };

    // let shared = Shared {
    //     inner: Mutex::new(Inner {
    //         queue: VecDeque::default(),
    //         senders: 1,
    //     }),
    //     available: Condvar::new(),
    // };
    let shared = Arc::new(shared);
    (
        Sender {
            shared: shared.clone(),
        },
        Receiver {
            shared: shared.clone(),
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ping_pong() {
        let (mut tx, mut rx) = channel();
        tx.send(11);
        assert_eq!(11, rx.recv().unwrap())
    }

    #[test]
    fn drop_test() {
        let (tx, mut rx) = channel::<()>();
        // let _ = tx;
        drop(tx);
        assert_eq!(rx.recv(), None)
    }
}
