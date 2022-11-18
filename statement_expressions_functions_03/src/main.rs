fn main() {
    // rust 函数由多个语句组成，最后有一个表达式来返回值
    // rust 中将语句与表达严格区分
    println!("{}", add_with_extra(24, 42));

    // 无法将let语句复制给其他值
    // let b = (let a = 8);

    // 表达式如果不返回任何值，就回隐式的返回一个 ()
    let x = 23;
    assert_eq!(ret_unit_type(x), ());

    let b = 5u32;

    //  c 等于 b_cube + b_squared + b 表达式的值
    let c = {
        let b_squared = b * b;
        let b_cube = b_squared * b;
        b_cube + b_squared + b
    };

    // 加上分号就成了语句，不会返回2 * b的值，隐式返回语句的值 ‘()’
    let d = {
        //  b + 2;
        let _ = b + 2;
    };

    println!("b is {:?}", b);
    println!("c is {}", c);
    println!("d is {:?}", d);

    // rust 中函数也是表达式，也会有返回值
    let x1 = plus_or_minus(6);
    println!("the value of x1 is : {}", x1);

    // 发散函数(diverge function) 当用 ‘!’作为返回值时，就表示为发散函数，一般用于导致程序崩溃的函数
    // dead_end()

    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);
    print()
}

fn print() -> () {
    println!("hello,world");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}
// fn dead_end() -> ! {
//     panic!("panic")
// }
fn add_with_extra(x: i32, y: i32) -> i32 {
    let x = x + 1; // 语句
    let y = y + 3; // 语句
                   // 表达式后加上分号，它就会变成一条语句，再也不会返回一个值
    x + y // 表达式
}

fn ret_unit_type(x: i32) {
    // if 语句也是一个表达式，因此可以用于赋值，也可以直接返回
    let y = if x % 2 == 1 { "odd" } else { "even" };
    println!("x is {}", y)
}

// return 也可以作为返回值
fn plus_or_minus(x: i32) -> i32 {
    if x > 5 {
        return x - 5;
    }

    x + 5
}
