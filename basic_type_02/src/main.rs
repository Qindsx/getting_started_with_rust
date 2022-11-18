use num::complex::Complex;
use std::mem::size_of_val;
use std::ops::{Range, RangeInclusive};

fn main() {
    // // 编译器会进行自动推导，给予twenty i32的类型
    // let twenty = 20;
    // // 类型标注
    // let twenty_one: i32 = 21;
    // // 通过类型后缀的方式进行类型标注：22是i32类型
    // let twenty_two = 22i32;

    // // 只有同样类型，才能运算
    // let addition = twenty + twenty_one + twenty_two;
    // println!(
    //     "{} + {} + {} = {}",
    //     twenty, twenty_one, twenty_two, addition
    // );

    // // 对于较长的数字，可以用_进行分割，提升可读性
    // let one_million: i64 = 1_000_000;
    // println!("{}", one_million.pow(2));

    // // 定义一个f32数组，其中42.0会自动被推导为f32类型
    // let forty_twos = [42.0, 42f32, 42.0_f32];

    // // 打印数组中第一个值，并控制小数位为2位
    // println!("{:.2}", forty_twos[0]);

    // // 二进制为00000010
    // let a: i32 = 2;
    // // 二进制为00000011
    // let b: i32 = 3;

    // println!("(a & b) value is {}", a & b);

    // println!("(a | b) value is {}", a | b);

    // println!("(a ^ b) value is {}", a ^ b);

    // println!("(!b) value is {} ", !b);

    // println!("(a << b) value is {}", a << b);

    // println!("(a >> b) value is {}", a >> b);

    // let mut a = a;
    // // 注意这些计算符除了!之外都可以加上=进行赋值 (因为!=要用来判断不等于)
    // a <<= b;
    // println!("(a << b) value is {}", a);

    // // range
    // // for i in 1..5 { // 1,2,3,4
    // // for i in 1..=5 { // 1,2,3,4,5
    // for i in 'a'..='z' {
    //     println!("{}", i)
    // }

    // //Rust 数值库：num。
    let a_1 = Complex { re: 2.1, im: -1.2 };
    let b_1 = Complex::new(11.1, 22.2);
    let result = a_1 + b_1;

    println!("{} + {}i", result.re, result.im);

    //编译器自动推导一个类型
    let x: i32 = 5;
    let y = 5; // 如果是一个整形 默认为i32类型

    let v: u16 = 38_u8 as u16;

    println!("{},{},{}", x, y, v);

    let x_1 = 5;
    assert_eq!("i32".to_string(), type_of(&x_1));

    // i 表示有符号 u表示无符号
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);
    assert_eq!(i8::MIN, -128);

    let v1 = 247_u8 + 8;
    let v2 = i8::checked_add(119, 8).unwrap();
    println!("v1 = {},v2 = {}", v1, v2);

    let v3 = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v3 == 1597);

    // 浮点数
    assert!(0.1_f32 + 0.2_f32 == 0.3_f32);

    // range
    let mut sum = 0;
    for i in -3..=2 {
        sum += i;
    }
    assert!(sum == -3);

    for c in 'a'..='z' {
        println!("{}", c as u8);
    }

    // Range库
    assert_eq!((1..5), Range { start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    // 整数加法
    assert!(1u32 + 2 == 3);

    // 整数减法
    assert!(1i32 - 2 == -1);
    assert!(1i8 - 2 == -1);

    assert!(3 * 50 == 150);

    assert!(9 / 3 == 3); // error ! 修改它让代码工作

    assert!(24 % 5 == 4);

    // 逻辑与或非操作
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // 位操作
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // 字符 char
    // 所有字符都是四个字节 支持Unicode  范围 U+0000 ~ U+D7FF 和 U+E000 ~ U+10FFFF
    let c1 = 'a';
    assert_eq!(size_of_val(&c1), 4);

    let c2 = '铯';
    assert_eq!(size_of_val(&c2), 4);

    let c3 = '钛';
    print_char(c3);

    // 布尔 bool
    let _b = true;
    let b = true;
    if b {
        println!("b is true");
    }

    let b1 = false;
    let b2 = true && false;
    assert_eq!(b1, b2);

    //单元类型 ()
    let _u = ();
    let u = ();
    // 函数如果没有返回值就会返回一个单元类型`()`
    // main 函数同样返回一个单元类型
    assert_eq!(u, implicitly_ret_unit());

    let u1 = ();
    // 单元类型内存为空 相当于空 可以用来占位
    println!("unit size is {}", size_of_val(&u1));
}

// 以下函数可以获取传入参数的类型，并返回类型的字符串形式，例如  "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

fn print_char(c: char) {
    println!("c is {}", c);
}

fn implicitly_ret_unit() {
    println!("I will return a ()")
}
// fn explicitly_ret_unit() -> () {
//     println!("I will return a ()")
// }
