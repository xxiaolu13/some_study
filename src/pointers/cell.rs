use  std::cell::UnsafeCell;
// 主要是两个问题，对于cell
// 1. 实现了!sync，因为value类型是unsafecell，自动实现!sync
// 2. 内部值需要实现copy，避免悬垂引用的情况

//在 Rust 中，从共享引用（&T）正确转换为排他引用（&mut T）的唯一方法是使用 unsafe cell。
//改变值需要&mut T，但是在有&T的情况下就需要使用cell了
pub struct Cell<T>{
    value: UnsafeCell<T>,
}
impl<T> Cell<T>{
    pub fn new(value: T) -> Self{
        Cell{
            value: UnsafeCell::new(value)
        }
    }
    pub fn set(&self,value: T){
        unsafe {*self.value.get() = value};
    }
    pub fn get(&self) -> T  // use test bad2
    where 
        T: Copy,
    {
        unsafe {*self.value.get()}
    }
    // pub fn get(&self) -> &T{
    //     unsafe {&*self.value.get()}
    // }
}
// unsafe impl<T> Sync for Cell<T>{} // use test bad

#[cfg(test)]
mod tests {
    use super::Cell;
    #[test]
    fn test1(){
        let c = Cell::new(10);
        let d = c.get();
        c.set(11);
        let f = c.get();
        assert_eq!((d,f),(10,11))
    }
//     #[test] // when sync
//     fn bad() {
//         let test_cell = std::sync::Arc::new(Cell::new(0));
//         let x = std::sync::Arc::clone(&test_cell);
//         let t1 = std::thread::spawn({ move||
//             for _ in 0..10000{
//                 let a = x.get();
//                 x.set(a+1);
//             }
//         });
//         let y = std::sync::Arc::clone(&test_cell);
//         let t2= std::thread::spawn({move||
//             for _ in 0..10000{
//                 let a = y.get();
//                 y.set(a+1);
//             }
//         });
//         t2.join().unwrap();
//         t1.join().unwrap();
//         assert_eq!(*test_cell.get(),20000)
//     }
// //     时间 | 线程1              | 线程2              | Cell的值
// // -----|-------------------|-------------------|----------
// // T1   | a = get() = 5     |                   | 5
// // T2   |                   | b = get() = 5     | 5
// // T3   | set(5+1) = 6      |                   | 6
// // T4   |                   | set(5+1) = 6      | 6  ← 丢失了一次+1！

//     #[test] // when get() not add T: Copy
//     fn bad2() {
//         let c = Cell::new(String::from("hello"));
//         let x = c.get();
//         let y = c.set(String::from("This is empty"));
//         eprintln!("{}",x)
//     }

}


