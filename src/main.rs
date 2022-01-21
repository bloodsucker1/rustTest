mod test1;

fn main() {
    // 默认用法
    print!("{}", 1);
    // 八进制
    println!("{:o}", 9);
    // 十六进制 小写
    println!("{:x}", 255);
    // 十六进制 大写
    println!("{:X}", 255);
    // 指针
    println!("{:p}", &0);
    // 二进制
    println!("{:b}", 15);
    // 科学计数(小写)
    println!("{:e}", 10000f32);
    // 科学计数(大写)
    println!("{:E}", 10000f32);
    // 打印Debug
    println!("{:?}", "test");
    // 带换行和缩进的Debug打印
    println!("{:#?}", ("test1", "test2"));
    // 命名参数
    println!("{a} {b} {b}", a = "x", b = "y");
}
