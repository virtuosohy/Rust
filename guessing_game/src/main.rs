use rand::Rng;
use std::cmp::Ordering; // *标准库中的枚举
use std::io; // *trait  其他语言里的接口

fn main() {
    println!("猜数");

    // thread_rng 随机数生成器
    let secret_number = rand::thread_rng().gen_range(1..101); // *包括1 不包括101

    println!("数字是: {}", secret_number);

    loop {
        println!("猜测一个数");

        // * mut 在变量前，让变量可以修改
        // *RHS 创建一个惯用函数名5(⊙o⊙)…saz
        //*这句话就是定义了一个可变的变量 然后把他的值绑定到空的字符串实例上面
        let mut guess = String::new();

        // *read_line方法获取用户输入
        // * read_line方法无论用户输入什么 都会进行读取  有返回值 类型是 io::Result<usize>   result是枚举类型
        // * io::Result    OK,Err    Err里面附带失败的原因
        //*  expect是result定义的方法    假如result返回的是Err，expert就会中断当前程序 并将传入的字符串的值显示   OK类似
        io::stdin().read_line(&mut guess).expect("无法被读取");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // * 花括号是站位符，输出时被替换为后面变量的值
        println!("你猜测的是： {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!(" you get");
                break;
            }
        }
    }
}
