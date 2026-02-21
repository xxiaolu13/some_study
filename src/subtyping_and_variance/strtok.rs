// 观察it work 和 it work2，这两个test为这次讨论内容
// it work中
// 首先定义mut 的 a 然后 let hello 调用 strtok，这里面 strtok需要'a 然而 传入的a是 'static ， 'static 可以转化成为'a,
// 然而hello在{}作用域内，但是出了{}没有自动释放，导致最后的assert报错，因为assert需要借用 a
// it work2 中
// 通过check is static 可以看出  strtok这里面的'a不行了，因为他需要一个'static的生命周期的&str
// == 上面说的有点乱
// subtype的概念，子类型
// Class fruit
// class apple: fruit
// apple 满足的功能更加多，更有用，apple是fruit的子类型
// &'static &'a 这俩 &'static 要比 &'a 更加有用， static 是 a 的 子类型
// Fn(_: &'static) Fn(_: &'a) 这两个 Fn a 要比 Fn static 更有用了， Fn a  是 Fn static 子类型  => 逆变
//
//
// 根因是啥呢，&mut T,其中在 &mut 也就是外层，这块是协变的，然后 在T这块是不协变的
// 在测试中，strtok只有一个生命周期，所以出现了这个问题
pub fn strtok<'a, 'b>(s: &'a mut &'b str, delimiter: char) -> &'b str {
    if let Some(i) = s.find(delimiter) {
        let prefix = &s[..i];
        let suffix = &s[(i + delimiter.len_utf8())..];
        *s = suffix;
        prefix
    } else {
        let prefix = *s;
        *s = "";
        prefix
    }
}
// pub fn strtok<'a>(s: &'a mut &'a str, delimiter: &'a str) -> &'a str {
//     //s: &'a mut &'a str 他的类型是指针？还是&str的可变引用
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_work() {
        // let mut <'a> a = "hello world";
        let mut a = "hello world";
        // 问题1  这里面 let x = "xx" x 是 'static 使用strtok时，'a 会 变成 'static 这样strtok返回的东西会是'static的生命周期
        {
            let hello = strtok(&mut a, ' ');
            assert_eq!(hello, "hello");
        }
        assert_eq!(a, "world");
    }

    #[test]
    fn it_work_2() {
        fn check_is_static(_: &'static str) {}

        let mut a = "hello world";

        check_is_static(a);
        let _ = strtok(&mut a, ' ');

        assert_eq!(a, "world");
    }
}
