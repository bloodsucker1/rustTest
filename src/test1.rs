#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!("{}", 2)
    }

    #[test]
    fn test1() {
        // 这段代码只是掩饰变量遮蔽功能，并不是Vec类型的最佳初始化方法
        // v 必须是mut修饰,因为我们需要对它写入数据
        let mut v = Vec::new();
        v.push(1);
        v.push(2);
        v.push(3);
        // 从这里往下，v成了只读变量，可读写变量v已经被遮蔽，无法访问
        let v = v;

        for i in &v {
            println!("{}", i);
        }
    }


    #[test]
    fn test2() {
        // 局部变量声明,可以留待后面初始化
        let x;
        let y = 1_i32;
        x = 2_i32;
        println!("{} {}", x, y);
        // 全局变量必须声明的时候初始化，因为全局变量可以写到函数外面，被任意一个函数使用
        static G1: i32 = 3;
        println!("{}", G1);
        // 可变全局变量无论读写都必须用unsafe修饰
        static mut G2: i32 = 4;
        unsafe {
            G2 = 5;
            println!("{}", G2);
        }
        // 全局变量的内存不是分配在当前函数栈上，函数退出的时候，并不会销毁全局变量占用的内存空间，程序退出才会回收
    }

    #[test]
    fn test3() {
        let c1 = '\n'; // 换行符
        let c2 = '\x7f'; // 8 bit字符变量
        let c3 = '\u{7FFF}'; // unicode字符
        // Rust提供了单字节字符字面量来表示ASCII字符
        // 可以使用一个字母b在字符或者字符串前面，代表这个字面量存储在u8类型数组中，这样占用空间比char型数组要小一些
        let x: u8 = 1;
        let y: u8 = b'A';
        let s: &[u8; 5] = b"hello";
        let r: &[u8; 14] = br#"hello \n world"#;
        println!("{} {} {} {} {} {:?} {:?} ", c1, c2, c3, x, y, r, s);
    }

    #[test]
    fn test4() {
        let var1: i32 = 32; //十进制表示
        let var2: i32 = 0xFF; // 以0x开头代表十六进制表示
        let var3: i32 = 0o55; // 以0o开头代表八进制表示
        let var4: i32 = 0b1001; // 以0b开头代表二进制表示
        let var5: i32 = 0x_1234_ABCD; // 使用下划线分割数字,不影响语义，但是极大地提升了阅读体验
        // 字面量后面可以跟后缀，可代表该数字的具体类型，从而省略显示类型标记
        let var6 = 123usize; // 变量是usize类型
        let var7 = 0x_ff_u8; // 变量是u8类型
        let var8 = 32;  // 不写类型,默认为i32类型
        println!("{} {} {} {} {} {} {} {}", var1, var2, var3, var4, var5, var6, var7, var8);

        // 在Rust中，我们可以为任何一个类型添加方法，整型也不例外。比如在标准库中，整数类型有一个方法是pow，它可以计算n次幂
        let x: i32 = 9;
        println!("9 power 3={}", x.pow(3));
        // 同理，我们甚至可以不使用变量，直接对整型字面量调用函数
        print!("9 power 3={}", 9_i32.pow(3));
    }

    #[test]
    fn test5() {
        let x = 10;
        let y = x * x;
        println!("{}", y);
    }


    #[test]
    fn test6() {
        let i = 100_i8;
        println!("checked {:?}", i.checked_add(i));
        println!("saturating {:?}", i.saturating_add(i));
        println!("wrapping {:?}", i.wrapping_add(i));
    }
}