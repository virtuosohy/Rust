# rust

## 一 .环境准备：

### 解决link-exe not find

> 在终端中输入

```bash
rustup toolchain install stable-x86_64-pc-windows-gnu

rustup default stable-x86_64-pc-windows-gnu
```









### Hello world

> 代码

```rust
fn main() {
    println!("Hello, world!");
}

```

#### 程序剖析

1.定义函数  fn main(){}   没有参数，没有返回

2.main函数是每个rust可执行程序最先运行的代码

3.打印文本： println!("Hello, world!")

- rust缩进是4个空格

- println！是一个 Rust macro（宏）

  如果是函数就没有 **！**   字符串用！输出

  ​



### cargo

cargo是Rust的构建系统和包管理系统

#### 命令顺序：

1. cargo new 项目名
2. cargo build
3. cargo run



#### Cargo.toml

TOML:是cargo 的配置格式

package 布置

--name：项目名

--version：项目版本

--author：作者

--edition：使用的Rust版本

dependencies 依赖项

在Rust里面，代码的包称为crate







## 二 .猜数游戏

> 游戏目标

- 生成一个1-100的数
- 提示玩家进行猜测
- 猜完过后，显示偏大还是偏小
- 正确，退出




### 第一部分

#### 具体操作


```rust
use std::io;

fn main() {
    println!("猜数");

    println!("猜测一个数");

    // * mut 在变量前，让变量可以修改
    // *RHS 创建一个惯用函数名
    //*这句话就是定义了一个可变的变量 然后把他的值绑定到空的字符串实例上面
    let mut guess = String::new();

    
    //用 io 包下的 stdin 函数来获取用户输入
    // *read_line方法获取用户输入
    // * read_line方法无论用户输入什么 都会进行读取  有返回值 类型是 io::Result<usize>   result是枚举类型
    // * io::Result    OK,Err    Err里面附带失败的原因
    //*  expect是result定义的方法    假如result返回的是Err，expert就会中断当前程序 并将传入的字符串的值显示   OK类似
    io::stdin().read_line(&mut guess).expect("无法被读取");

   
   // * 花括号是站位符，输出时被替换为后面变量的值
   
   println!("你猜测的是： {}", guess);

}

```

#### 程序剖析

1. `use std::io;   `   调用io库
2. `let mut guess`  变量本来不能变化，若要使变量变化在变量前加`mut
3. `String::new()`   创建一个函数名，将刚刚的可变变量绑定到空的字符串实例上面
4. `io：：stdin`使用包里面的函数获取用户输入
5. `read_line`        `stdin` 函数返回了一个 io::Stdin 对象，调用它的 `read_line` 方法就能够读取到用户的输入
6. 无论用户输入什么 都会进行读取  有返回值 类型是 io::Result<usize>
5. io::Result    OK,Err    Err里面附带失败的原因
6.  expect是result定义的方法    假如result返回的是Err，expert就会中断当前程序 并将传入的字符串的值显示   OK类似


### 第二部分--生成数字

#### 具体操作

1. 在cargo.toml的dependencies中声明rand库
2. 在main.rs文件开头处使用`use rand::Rng;`使用库

```rust
// thread_rng 随机数生成器
    let secret_number = rand::thread_rng().gen_range(1..101); // *包括1 不包括101
```

#### 程序剖析

1. `rand::thread_rng()`  随机数生成器
2. `gen_range(1..101)`   rng里面的方法 后面接俩个参数 （包括第一个，不包括第二个）




> 目前全部代码：

```rust
use rand::Rng;
use std::io; // *trait  其他语言里的接口

fn main() {
    println!("猜数");

    // thread_rng 随机数生成器
    let secret_number = rand::thread_rng().gen_range(1..101); // *包括1 不包括101

    println!("数字是: {}", secret_number);

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

    // * 花括号是站位符，输出时被替换为后面变量的值

    println!("你猜测的是： {}", guess);
}

```



### 第三部分--比较猜测数字和生成数字

#### 具体操作

1.调用标准库中的枚举方法

```rust
use std::cmp::Ordering; // *标准库中的枚举
```

2.进行匹配

```rust
match guess.cmp(&secret_number) {
        Ordering::Less => println!("too small"),
        Ordering::Greater => println!("too big"),
        Ordering::Equal => println!(" you get"),
    }
```

3.因为guess是字符串，而number是整型，所以会报错

4所以在前面将guess转化为整型

```rust
 let guess: u32 = guess.trim().parse().expect("输入一个数字");
```

#### 程序剖析

`trim（）`  去掉俩端的空白

`pause（）`解析成整数

ruo失败就调用`expert`方法，输出提示话语



> 目前全部代码

```rust
use rand::Rng;
use std::cmp::Ordering; // *标准库中的枚举
use std::io; // *trait  其他语言里的接口

fn main() {
    println!("猜数");

    // thread_rng 随机数生成器
    let secret_number = rand::thread_rng().gen_range(1..101); // *包括1 不包括101

    println!("数字是: {}", secret_number);

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

    let guess: u32 = guess.trim().parse().expect("输入一个数字");

    // * 花括号是站位符，输出时被替换为后面变量的值
    println!("你猜测的是： {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("too small"),
        Ordering::Greater => println!("too big"),
        Ordering::Equal => println!(" you get"),
    }
}

```





### 第四部分--允许多次猜测

1.使用循环函数,将以前的代码放入循环中

```rust
loop{
    
    
}
```

2.在数字匹配的那一行最后加`break`结束循环

```rust
 Ordering::Equal => {
                println!(" you get");
                break;
            }
```





3.此处会有一个问题，假如用户输入非数字，程序会崩溃报错

```rust
let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
```

result里面有OK和err  ，在err中使用下划线表示不关心错误信息，并continue让用户重新输入



### 完整代码

```rust
use rand::Rng;
use std::cmp::Ordering; // 标准库中的枚举
use std::io; // *trait  其他语言里的接口

fn main() {
    println!("猜数");

    // thread_rng 随机数生成器
    let secret_number = rand::thread_rng().gen_range(1..101); // *包括1 不包括101

    //println!("数字是: {}", secret_number);

    loop {
        println!("猜测一个数");

        //  mut 在变量前，让变量可以修改
        // RHS 创建一个惯用函数名5(⊙o⊙)…saz
        //这句话就是定义了一个可变的变量 然后把他的值绑定到空的字符串实例上面
        let mut guess = String::new();

        // read_line方法获取用户输入
        // read_line方法无论用户输入什么 都会进行读取  有返回值 类型是 io::Result<usize>   result是枚举类型
        // io::Result    OK,Err    Err里面附带失败的原因
        //expect是result定义的方法    假如result返回的是Err，expert就会中断当前程序 并将传入的字符串的值显示   OK类似
        io::stdin().read_line(&mut guess).expect("无法被读取");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // 大括号是站位符，输出时被替换为后面变量的值
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

```





## 三.通用编程概览

### 3.1 变量和可变性

- 声明变量使用let关键字
- 默认情况下，变量是不可变的
- 在声明变量时在变量前添加mut 可使变量变化

#### 常量和变量

##### 常量

常量（constant） ，不可变，但和不可变的变量有区别：

-声明常量使用const，他的类型必须被标注

-常量可在任何作用域内进行声明，包括全局作用域

-常量只可以绑定常量表达式，无法绑定到函数调用结果或只能在运行时才能计算出的值

命名规范：rust中使用**全大写字母**   每个字母间用下划线分开

##### shadowing（隐藏）

可以使用相同的名字声明新的变量，新的变量会shadow之前声明的同名变量



### 3.2数据类型

#### 标量类型

4个主要的标量类型：整数，浮点，布尔，字符

#### 复合类型

分为元组（tuple）和数组

##### tuple

tuple可以将多个值放在一个类型里

tuple的长度是固定的，一旦声明就无法改变

**创建tuple**

创建在小括号里，值用逗号隔开

> 例子：

```rust
let tup:(i32,f64,u8) = (100,3.0,1);
printl!("{},{},{}",tup.0,tup.1,tup.2)
```



##### 数组

tuple可以将多个值，（类型必须相同）放在一个类型里

tuple的长度是固定的，一旦声明就无法改变



### 3.3函数

- 声明函数使用fn关键字

- 按照惯例，函数命名，所有的字母都是小写，单词之间用下划线分开

- 函数的返回值：在=>符号后面生命函数返回值的类型

- > 例子

  ```
  fn five () => i32{
      5
  }

  fn main (){
      let x = five();
      println!{"x is {}",x}
  }
  ```

  ​



### 3.4循环

#### loop

loop循环会反复执行大括号里面的语句

> 例

```rust
fn main (){
    
    let mut num = 0;
    
    let result  = loop {
        num += 1;
        
        if num ==10 {
            break num * 2；
        }
    }
    println("结果是"，result)；
}
```





#### while

在每次执行循环前都判断一次条件

> 例

```rust
fn main (){
    
    let mut num = 3;
    
    whike num != 0 {
        println!("{}"!,num);
        
        num = num -1;
    }
    println("LIFTOFF")；
}
```

> 输出

```rust
3!
2！
1！
LIFTOFF
```





#### for循环

> 例

```rust
fn main (){
    
    let a = [10, 20, 30, 40, 50 ];
    for element in a.iter(){
         println("数的值是 {}" ， element)；
    }
}
```



`iter`方法返回一个可遍历thing



#### range

- 标准库提供
- 指定一个开始数字和结束数字，range可以生成他们之间的数字（不含结束 ）
- rev方法可以反转range



for循环实现倒计时例子

```rust
fn main (){
    
    for number in(1..4).rev(){
        println("{}!" , number);
    }
         println("LIFTOFF")；
    }
}
```



## 四.所有权

- 所有权是rust最独特的特性，它让rust无需GC就可以保证内存安全


- 所有程序在运行时都必须管理他们使用计算机内存的方法（其他语言有垃圾收集机制，或者是程序员显式地分配和释放内存）



- rust采用的方法是：内存通过一个所有权系统来管理，其中包括一组编译器在编译时检查的规则


- 所以说当程序运行时，所有权特性不会减慢程序运行的速度





### Stack  vs Heap

> 栈内存和堆内存

#### **stack**

- Stack 按值的接受顺序来存储，按相反的顺序将他们移除 （后进先出，LIFO）

--添加数据叫压入栈

--移除数据叫弹出栈

- 所有存储在Stack上的数据必须拥有已知的固定的大小（编译时大小未知的数据或大小不固定的数据存放在heap里）

所有权存在的原因 管理heap数据



### 所有权规则

- 每个值都有一个变量，这个变量是该值的所有者
- 每个值同时只能有一个所有者
- 当所有者超出作用域的时候，该值会被删除





### sring类型

- 字符串字面值：程序里手写的字符串值，他们是不可变的

- 存放在heap里，能够存储在编译时未知数量的文本

- 使用from函数从字符串字面值创建出String类型、

- > 例  let s = String::from("hello")



### 克隆和复制

```rust
fn main (){
    let a1 = String::from("hello")
    let a2 = a1
    
    println!("{}",a1)
}
```



**会报错**

因为hello存储在heap上，将a1赋值给a2后，a1就会失效



> 使用克隆方法可以解决这个问题，针对heap

```rust
fn main (){
    let a1 = String::from("hello")
    let a2 = a1.clone()
    
    println!("{}",a1)
}
```





对于steak的情况 复制即可完成

```
fn main (){
    let x = 1
    let y = x
    
    println!("{}",x)
}
```







### 引用和借用

引用：& 符号就是表示引用：允许你引用某些值而不取得其的所有权

> 例

```rust
fn main (){
    let s1 = string::from("hello");
    let len  = calculate_length(&s1);
    
    println!("the length of '{}' is {}",s1 , length);
}

fn calculate_length(s::&string) => usize {
    s.len()
}
```





可变借用

```rust
fn main (){
    let mut s1 = string::from("hello");
    let len  = calculate_length(&mut s1);
    
    println!("the length of '{}' is {}",s1 , length);
}

fn calculate_length(s: &mut string) => usize {
    s.push_str(",world")
    s.len()
}
```





### 切片

#### 字符串切片

- 字符串切片是指向字符串中一部分内容的引用

- > 例
  >
  > ```
  > fn main (){
  >     let s = String::from("hello world")
  >     
  >     let hello = &s[0..5];
  >     let world = &s[6..11];
  >     let whole = &s[..]
  > }
  > ```
  >
  > ​






## 五.结构体 struct

--自定义的数据类型

### 定义struct

- 使用struct关键字，并为整个struct命名

- 在花括号内，为所有字段（field）定义名称和类型

- > 例
  >
  > ```rust 
  > struct User{
  >     username：String
  >     
  > }
  > ```
  >
  > ​

### 实例化struct

需要创建struct实例：

-每个字符指定具体值

```rust
struct User{
    username：String
    
}

fn main (){
    
    let user1 = uesr {
        
        username:string::from("a")
    }
}
```





### 获取struct里面的某个值

使用点标记法：

> 例

```rust
 
    let mut user1 = uesr {
        
        username:string::from("a")
        
    }
    
    user1.username = string::from("b")
```



当字段名和字段值对应变量名相同时，可以使用字段初始化简写



```rust 
User{
    name;
    email;
 sex:male ； 
}
```





### struct的更新语法

```rust
let user2 = User {
    
    class:1;
    ..user1
}
```





### Tuple struct

--tuple struct整体有名，但里面的元素没有名

定义tuple struct：使用struct关键字，后面是名字，以及元素的类型

> 例34

```rust 
struct Color（i32，i32，i32）
struct Point（i32，i32，i32)
let black = Color（0,0,0）
let red = Point（0,0,0）
//black和red是不同的类型，不同的tuple struct实例
```





struct例子--计算长方形面积

```rust
fn main (){
    let w = 30
    let l = 50 
    println!("{}",area(w,l))
    
}

fn area(width:u32 ,length:u32) => u32 {
    
    width * length
    
}
```







tuple方法(将长和宽联系起来)

```rust
fn main (){
    let rect = (30,50)
    println!("{}",area(rect))
    
}

fn area(dim: (u32,u32)) => u32 {
   dim.0 * dim.1
    
}
```





结构体方法

```rust 
#[derive(Debug)]

struct Rectangle {
    width: u32,
    length:u32,
}

fn main (){
    let rect = Rectangle {
        width:30,
        length:50,
    };
    
    println!("{}",area(&rect)) //1500
    println!("{:?}",rect)  // Rectangle {width:30,length:50,};
    
}

fn area(rect:&Rectangle) => u32 {
   rect.width * rect.length
    
}
```



#### 代码解析

1. **函数参数借用**：在 `area` 函数的定义中，参数 `rect` 被定义为 `&Rectangle` 类型，这意味着我们借用了一个 `Rectangle` 实例的引用。这里的借用是不可变的，因为我们没有在函数内部修改 `Rectangle` 的任何字段。使用不可变借用可以避免在函数调用期间修改原始数据，同时允许我们读取数据。
2. **在 println! 宏中借用**：在 `main` 函数中，调用 `area` 函数时传递了 `&rect`。这里，我们借用了 `rect` 实例的引用作为 `area` 函数的参数。这允许 `area` 函数访问 `main` 函数中创建的 `Rectangle` 实例，而不需要将所有权转移给 `area` 函数。
3. 开头添加`#[derive(Debug)]`是为了避免输出数据时报错



### struct方法

第一个参数是self，表示方法被调用的struct实例

需要在**impl**里面定义

```rust 
#[derive(Debug)]

struct Rectangle {
    width: u32,
    length:u32,
}

impl Rectangle {
    fn area(&self) => u32 {
  self.width * self.length 
}
}
fn main (){
    let rect = Rectangle {
        width:30,
        length:50,
    };
    
    
    println!("{:?}",rect.area())
    
}


```









## 六.枚举与模式匹配
