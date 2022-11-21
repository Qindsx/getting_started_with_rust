fn main() {
    // 借用规则如下
    //同一时刻，你只能拥有要么一个可变引用, 要么任意多个不可变引用
    //引用必须总是有效的

    // 常规应用是一个指针类型，指向对象的内存地址
    // 创建一个x值的应用 y ，使用解引用运算符来解出 y 所使用的值
    let x = 5;
    let y = &x;

    println!("{},{}", x, *y);

    // 将引用作为参数传递给函数，而不是将所有权转移给函数
    // 不需要通过参数传入所有权，然后通过函数返回传递所有权，这样更加简洁
    // calculate_length()的参数的类型不再是 String ,而是 &String
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // 引用如同变量一样，默认的引用指向也是不可变的，需要加上 &mut 创建一个可变引用
    // 用一作用域，特定数据只能有一个可变引用

    let mut s = String::from("test1");
    {
        let r1 = &mut s;
        println!("{}", r1);
    }
    let r2 = &mut s;
    println!("{}", r2);

    //可变引用与不可变引用不能同时存在
    let r3 = &s;
    let r4 = &s;
    // 如果一个数据被借用，而且是不可边引用，如果有其他可变引用就会有错误
    // let r5 = &mut s;
    println!("{},{}", r3, r4);

    // 引用的作用域与变量的有所不同，引用的作用域一直到最后一次使用它的地方，而变量的作用域从创建到 }
    // 以下代码虽然违背了 可变引用与不可变引用不能同时存在 这一规则，但是可以正常使用
    // 对于这种编译器优化行为，Rust 专门起了一个名字 —— Non-Lexical Lifetimes(NLL)，
    // 专门用于找到某个引用在作用域(})结束前就不再被使用的代码位置。
    let r6 = &s;
    let r7 = &s;
    println!("r6 is {},r7 is {}", r6, r7);

    //引用作用域的结束位置从花括号变成最后一次使用的位
    let r8 = &mut s;
    println!("r8 is {}", r8);

    //悬垂引用(Dangling References)也叫做悬垂指针，意思为指针指向某个值后，这个值被释放掉了，而指针仍然存在，其指向的内存可能不存在任何值或已被其它变量重新使用。
    // rust 可以保证这种悬垂引用永远不会出现
    let reference_to_nothing = dangle();
    println!("reference_to_nothing of value is {}", reference_to_nothing);

    let x1 = 7;
    let y1 = &x;
    println!("{} 的内存地址为 {}", x1, y1);

    let x2 = 8;
    let y2 = &x2;
    assert_eq!(8, *y2);

    let mut x3 = String::from("x3,hello,");
    println!("{}", x3);
    borrow_object(&x3);

    let mut x4 = String::from("x4,hello,");
    push_str(&mut x4);
    println!("{}", x4);

    let mut x5 = String::from("x5,hello,");
    let y5 = &mut x5;
    y5.push_str("world");
    println!("{}", y5);

    //ref 与 & 类似，可以用来获取一个值的引用，但是它们的用法有所不同。
    let c = '中';
    let x6 = &c;
    // 填写空白处，但是不要修改其它行的代码
    let ref y6 = c;
    assert_eq!(*x6, *y6);
    println!("{},{}", get_addr(x6), get_addr(y6));
    assert_eq!(get_addr(x6), get_addr(y6));

    let x7 = String::from("test7");
    let y7 = &x7;
    let z7 = &x7;
    println!("{},{},{}", x7, y7, z7);

    //从不可变对象借用可变
    let mut x8 = String::from("x8,hello, ");
    borrow_object_1(&mut x8);

    // 从可变对象借用不可变

    let mut x9 = String::from("x9,hello, ");
    borrow_object(&x9);
    x9.push_str("world");

    // NLL
    let mut x10 = String::from("x10,hello,");

    let y10 = &mut x10;
    y10.push_str("world");
    // y10结束，x10可以继续被引用
    let z10 = &mut x10;
    z10.push_str("!");
    // z10结束

    // println!("{}", y10); // 这里使用了一个本应被丢弃的引用 y10,
    println!("{}", x10);

    let mut x11 = String::from("x11,world,");

    let y11 = &mut x11;
    // let z11 = &mut x11; // cannot borrow `x11` as mutable more than once at a time

    y11.push_str("world");
}

fn borrow_object_1(s: &mut String) {
    println!("{}world", s);
}

fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}

fn push_str(s: &mut String) {
    s.push_str("world");
}
fn borrow_object(s: &String) {
    println!("{}world", s);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
fn dangle() -> String {
    // fn dangle() -> &String {
    let s = String::from("hello");
    // &s // 返回字符串 s 引用
    s // s 的 所有权被转移给外面的调用者。
} // s 离开作用域并被丢弃。内存被释放
