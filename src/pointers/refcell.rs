use std::cell::Cell;
use std::cell::UnsafeCell;
use std::ops::Deref;
use std::ops::DerefMut;

// 例子：
// struct Person {
//     name: String,
//     age: RefCell<u32>, // 注意这里
// }
// let list: Vec<Person> = vec![ Person { name: "Alice".into(), age: RefCell::new(20) } ];
// // 我们只能拿到不可变引用，因为 Vec 本身不是 mut 的
// let alice = &list[0];
// // 正常情况下，alice.name 是改不了的！因为 alice 是 &Person
// // alice.name = String::from("Bob"); // 编译报错！
// // 但是！通过 RefCell，我们可以改 age
// *alice.age.borrow_mut() = 21; // 成功！
// 这个例子很重要，如果 &x 确实无法修改x的值，但是如果有 & refcell（x） 通过borrow_mut 可以修改

#[derive(Copy, Clone)]
enum RefState {
    Unshared,
    Shared(isize),
    Excluesive, //独占 独家
}

pub struct Refcell<T> {
    value: UnsafeCell<T>,
    state: Cell<RefState>,
}

impl<T> Refcell<T> {
    pub fn new(value: T) -> Self {
        Refcell {
            value: UnsafeCell::new(value),
            state: Cell::new(RefState::Unshared),
        }
    }

    pub fn borrow(&self) -> Option<Ref<'_, T>> {
        match self.state.get() {
            RefState::Unshared => {
                self.state.set(RefState::Shared(1));
                // Some(unsafe {& *self.value.get()})
                Some(Ref { refcell: self })
            }
            RefState::Shared(num) => {
                self.state.set(RefState::Shared(num + 1));
                Some(Ref { refcell: self })
            }
            RefState::Excluesive => None,
        }
    }
    pub fn borrow_mut(&self) -> Option<RefMut<'_, T>> {
        match self.state.get() {
            RefState::Unshared => {
                self.state.set(RefState::Excluesive);
                // Some(unsafe {&mut *self.value.get()})
                Some(RefMut { refcell: self })
            }
            _ => None,
        }
    }
}

pub struct Ref<'refcell, T> {
    refcell: &'refcell Refcell<T>,
}
pub struct RefMut<'refcell, T> {
    refcell: &'refcell Refcell<T>,
}

impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Excluesive => unreachable!(),
            RefState::Unshared => unreachable!(),
            RefState::Shared(n) => {
                if n - 1 != 0 {
                    self.refcell.state.set(RefState::Shared(n - 1));
                } else {
                    self.refcell.state.set(RefState::Unshared);
                }
            }
        }
    }
}
impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Unshared | RefState::Shared(_) => unreachable!(),
            RefState::Excluesive => self.refcell.state.set(RefState::Unshared),
        }
    }
}

impl<T> Deref for Ref<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> Deref for RefMut<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.refcell.value.get() }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Barrier;

    use super::Refcell; // 使用你自己实现的 Refcell

    #[test]
    fn test_refcell_my() {
        let c = Refcell::new(5);
        let b = &c; // 这里获得的是不可变引用
        *b.borrow_mut().unwrap() = 10; // 不可变引用通过borrow_mut改变了值
        let val = unsafe { *b.value.get() };
        assert_eq!(val, 10)
    }

    #[test]
    fn zuoyongyu_test() {
        let mut c = 1;
        let b = &c;
        println!("{}", b);
        let _a = &mut c;
        println!("i32合理");
        //
        let c2 = Refcell::new(5);
        let b1 = c2.borrow().unwrap();
        println!("b1 is still alive: {:?}", *b1);
        let b2 = c2.borrow_mut();
        assert!(b2.is_none());
        //编译器视角：b1 是一个带有析构函数（Drop）的对象。Rust 规定，为了保证清理工作的确定性，所有实现了 Drop 的变量必须存活到作用域的末尾（即 } 处），除非你手动 drop(b1)。
        //运行时视角：当你调用 c2.borrow_mut() 时，b1 还没执行 drop。这意味着它持有的“借用计数”还没归还。此时 Refcell 的 state 依然是 Shared(1)。
        //结果：borrow_mut 检查 state 发现不是 Unshared，于是果断返回 None。
    }

    #[test]
    fn test_refcell_runtime_panic() {
        let c = Refcell::new(5);

        // 1. 拿到一个不可变借用
        let _b1 = c.borrow().unwrap();

        // 2. 尝试拿一个可变借用
        // 根据规则：有共享引用时，不能有排他引用
        let b2 = c.borrow_mut();

        // 在你的实现中，这会返回 None
        assert!(b2.is_none());

        println!("运行时成功拦截了违规借用！");
    }
}
