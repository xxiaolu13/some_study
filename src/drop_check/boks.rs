use std::{
    fmt::Debug,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

pub struct boks<T> {
    inner: *mut T,
    // t: PhantomData<T>,
}

// use in  ck1 and ck2
impl<T> Drop for boks<T> {
    fn drop(&mut self) {
        unsafe {
            // Box::from_raw(self.inner);
            drop(Box::from_raw(self.inner)); // 等价上面的  drop用于消除编译器的标黄
        }
    }
}
// use in ck4 当使用这个 ck4 会通过编译，但是使用Box不会过编译，引发讨论
unsafe impl<#[may_dangle] T> Drop for boks<T> {
    fn drop(&mut self) {
        unsafe {
            // Box::from_raw(self.inner);
            drop(Box::from_raw(self.inner)); // 等价上面的  drop用于消除编译器的标黄
        }
    }
}

impl<T> Deref for boks<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.inner }
    }
}

impl<T> DerefMut for boks<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.inner }
    }
}

impl<T> boks<T> {
    pub fn new(t: T) -> Self {
        boks {
            inner: Box::into_raw(Box::new(t)),
        }
    }
}

pub struct Oisann<T: Debug>(T);
impl<T: Debug> Drop for Oisann<T> {
    fn drop(&mut self) {
        println!("{:?}", self.0);
    }
}

// bok i32===
// 其实这里面写 i32 不会有报错了，drop check 指关注带生命周期的东西

// pub struct boksi32 {
//     inner: *mut &mut i32,
// }
// impl Drop for boksi32 {
//     fn drop(&mut self) {
//         unsafe { drop(Box::from_raw(self.inner)) }
//     }
// }
// impl boksi32 {
//     fn new_mut(t: &mut i32) -> Self {
//         boksi32 {
//             inner: Box::into_raw(Box::new(t)),
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn drop_ck1() {
        let mut t = 3;
        let b = boks::new(t);
        println!("{:?}", t);
    }
    #[test]
    fn drop_ck2() {
        let mut y = 3;
        let b = boks::new(&mut y);
        println!("{:?}", y);
        // 手写了drop（如果没手写就走NLL的逻辑了） 编译器认为：boks<T> 的 drop 实现可能会访问 T 的内容（因为是泛型），
        // 所以它要保证 b drop 的时候 &mut y 还有效，
        // 然而他会在 最后边的 } 进行drop，所以在 } 之前的 print 会报错
    }
    // #[test]
    // fn drop_ck3_for_boki32() {
    //     let mut y = 23;
    //     let b = boksi32::new(&mut y);
    //     println!("{:?}", y)
    // }

    // 如果没有 t: PhantomData<T>, 他会过编译（）， 正常是不应该过编译的 ，因为Oisann 的 drop使用了 内部的 T，
    // 所以需要加上 t: PhantomData<T>,，然而加上了他（也是协变）却会触发Oiann的dropcheck 如果不想要drop check，则用t: PhantomData<fn() -> T>,
    #[test]
    fn drop_ck4_for_oisnn() {
        let mut z = 23;
        let b = boks::new(Oisann(&mut z));
        println!("{:?}", z)
    }

    // 对比 ck5和ck6引发讨论，'static 给一个 短生命周期的 按理来说 ck5 应该过编译 类似ck6，但是目前没有过编译
    // 原因是 *mut T T没有协变，使用NonNull代替 *mut T 则有协变了
    //
    #[test]
    fn drop_ck5() {
        let s = String::from("hi");
        let mut bok1 = boks::new(&*s);
        let bok2: boks<&'static str> = boks::new("hi");
        bok1 = bok2;
    }

    #[test]
    fn drop_ck6() {
        let s = String::from("hi");
        let mut bok1 = Box::new(&*s);
        let bok2: Box<&'static str> = Box::new("hi");
        bok1 = bok2;
    }
}
