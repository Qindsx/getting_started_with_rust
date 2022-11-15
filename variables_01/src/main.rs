fn main() {
    //变量只有在初始化后才能被使用
    // 变量未被使用则会有相关警告
    let x = 32;
    let _y = 23;
    println!("x is equal to {}", x);

    //可以使用 mut 将变量标记为可变
    let mut num = 1;
    num += 2;
    println!("num ={}", num);

    //  作用域是一个变量在程序中能够保持合法的范围
    let x_1 = 12;
    {
        let y_1 = 34;
        println!("x 的值是 {}, y 的值是 {}", x_1, y_1);
    }
    // println!("x 的值是 {}, y 的值是 {}", x_1, y_1);

    // 函数也有内部的块级作用域
    let x_2 = define_x();
    println!("x_2 = {}", x_2);

    //若后面的变量声明的名称和之前的变量相同，则我们说：第一个变量被第二个同名变量遮蔽了( shadowing )
    let x_3 = 12;
    {
        let x_3 = 13;
        assert_eq!(x_3, 13);
    }
    assert_eq!(x_3, 12);

    let x_3 = 42;
    println!("x_3 = {}", x_3);

    // 下面这种用法看着很蠢，用着也很蠢，还有警告，狗都不用（
    let mut x_4 = 1;
    x_4 = 3;
    let mut x_4 = x_4;
    x_4 += 3;

    let y_4 = 4;
    let y_4 = "I can also be bound to text!";
    println!("x_4 = {}, y_4={}", x_4, y_4);

    // 变量解构
    // 我们可以将 let 跟一个模式一起使用来解构一个元组，最终将它解构为多个独立的变量
    //
    // let (x_5, y_5) = (1, 2); 错误用法
    // let (mut x_5, y_5) = (1, 2); //使用可变性
    let (x_5, y_5) = (1, 2);
    let x_5 = x_5 + 2; //变量遮蔽

    assert_eq!(x_5, 3);
    assert_eq!(y_5, 2);

    // 解构式赋值 可以在元组、切片和结构体中使用
    let (x_6, y_6);
    (x_6, ..) = (1, 3);
    [.., y_6] = [5, 6];

    assert_eq!([x_6, y_6], [1, 6]);
}

fn define_x() -> String {
    let x = "hello world".to_string();
    x
}
