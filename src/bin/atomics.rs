// 原子性操作，先实现了一个mutex => spin lock 自旋锁 （jonhoo 一直说不建议自旋锁）

use std::sync::atomic::Ordering;
use std::{
    cell::UnsafeCell,
    sync::atomic::AtomicBool,
    thread::{self, yield_now},
};

const LOCKED: bool = true;
const UNLOCKED: bool = false;

pub struct Mutex<T> {
    v: UnsafeCell<T>,
    locked: AtomicBool,
}

unsafe impl<T> Sync for Mutex<T> where T: Sync + Send {}

impl<T> Mutex<T> {
    fn with_lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        // while self.locked.load(std::sync::atomic::Ordering::Relaxed) != UNLOCKED {} // 一开始用这个，但是判断机制不是很原子
        // 使用compare exchange，代替load 和 store，compare exchange是一个原子操作 v
        while self
            .locked
            .compare_exchange_weak(UNLOCKED, LOCKED, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        //MESI protocol  其实是把内存位置分配给各个core？
        {
            while self.locked.load(Ordering::Relaxed) == LOCKED {
                thread::yield_now();
            }
            //
            // 锁的只读操作，用于阻塞（当锁被占用）,
            // 这块我理解是避免性能消耗，避免用compare exchange去循环，转而用 locked.load循环 节省一些资源 更优雅点
            //
            // 其实是节省性能消耗了 具体还是要看mesi协议，
            // 他的意思是 内存空间 分为 shared 和 exclusive，当用compare exchange 时候会exclusive，但是用load 他是只读的，仅仅读一下，保持了内存的shared
            //
            // weak 和 普通版本的区别，不想记了 大概是
            // 只需要单次尝试（不循环）→ 用 strong
            // 在循环中重试 → 用 weak
        }
        thread::yield_now();
        self.locked.store(LOCKED, Ordering::Relaxed);
        yield_now();
        let result = f(unsafe { &mut *self.v.get() });
        self.locked.store(UNLOCKED, Ordering::Release);
        result
    }
    fn new(t: T) -> Self {
        Mutex {
            v: UnsafeCell::new(t),
            locked: AtomicBool::new(UNLOCKED),
        }
    }
}

fn main() {
    let a: &'static _ = Box::leak(Box::new(Mutex::new(0)));
    let handles: Vec<_> = (0..10)
        .map(|_| {
            thread::spawn(move || {
                for _ in 0..1000 {
                    a.with_lock(|v| {
                        *v += 1;
                    })
                }
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    assert_eq!(a.with_lock(|v| *v), 1000 * 10);
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::AtomicUsize;

    use super::*;

    #[test]
    fn see_enum_for_order() {
        let x: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));
        let y: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));

        let t1 = thread::spawn(move || {
            let r1 = x.load(Ordering::Relaxed);
            y.store(42, Ordering::Relaxed);
            r1
        });
        let t2 = thread::spawn(move || {
            let r2 = y.load(Ordering::Relaxed);
            x.store(r2, Ordering::Relaxed);
            // relaxed 比较简单不保证顺序，比如 let r2 和 x.store 都是relax 顺序不会得到保证
            // Release：释放锁前，把之前的操作都"推出去" ,即确保release操作之前的任务都执行才会到release这步
            // Acquire：获取锁后，把之后的操作都"拦住",即确保Acquire操作之后的任务都执行才会Acquire这步
            r2
        });
        let r1 = t1.join().unwrap();
        let r2 = t2.join().unwrap();
        println!("r1: {}, r2: {}", r1, r2);
    }

    #[test]
    fn test_fetch() {
        let x: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
        let y: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
        let z: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));

        thread::spawn(|| {
            x.store(true, Ordering::Release);
        });
        thread::spawn(|| {
            y.store(true, Ordering::Release);
        });
        let r1 = thread::spawn(move || {
            while !x.load(Ordering::Acquire) {}
            if y.load(Ordering::Acquire) {
                z.fetch_add(1, Ordering::Relaxed);
            }
        });

        let r2 = thread::spawn(move || {
            while !y.load(Ordering::Acquire) {}
            if x.load(Ordering::Acquire) {
                z.fetch_add(1, Ordering::Relaxed);
            }
        });
        r1.join().unwrap();
        r2.join().unwrap();
        let z = z.load(Ordering::SeqCst);
        println!("{}", z)
    }
}
