fn main() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("xiaogao"),
        age: Box::new(21),
    };

    let x1 = String::from("test1");
    // let y1 = x1;
    let y1 = x1.clone();
    println!("{x1},{y1}");

    let x2 = String::from("test2");
    let y2 = take_ownership(x2);
    println!("{y2}");

    let x3 = give_ownership();
    println!("{x3}");

    let x4 = String::from("test4");
    // print_str(x4);
    // print_str(x4.clone());
    print_str(&x4);
    println!("{x4}");

    // 当元组内的元素都为 Copy 时，则该元组具有 Copy 特征
    let x5 = (1, 2, (), "test5");
    let y5 = x5;
    println!("{:?},{:?}", x5, y5);

    // 当所有权转移时，可变性也可以随之改变。
    let x6 = String::from("test6");
    let mut y6 = x6;
    y6.push_str("string");

    //当解构一个变量时，可以同时使用 move 和引用模式绑定的方式。当这么做时，部分 move 就会发生：变量中一部分的所有权被转移给其它变量，而另一部分我们获取了它的引用。
    //在这种情况下，原变量将无法再被使用，但是它没有转移所有权的那一部分依然可以使用，也就是之前被引用的那部分。
    let Person { name, ref age } = person;
    println!("The person's age is {}", age);
    println!("The person's name is {}", name);

    // error person的部分所有权被转移，无法访问完整person
    // println!("The person struct is {:?}", person);

    println!("The person's age from person struct is {}", person.age);

    let t1 = (String::from("test7"), String::from("test8"));
    let _s = t1.0;

    // t1的部分元素所有权被转移 无法访问完整 的t1
    // println!("{:?}", t1)
    println!("{:?}", t1.1);

    let t2 = (String::from("hello"), String::from("world"));
    let (s1, s2) = t2.clone();
    println!("{:?}, {:?}, {:?}", s1, s2, t2);
}

fn take_ownership(s: String) -> String {
    println!("test2");
    s
}

fn give_ownership() -> String {
    let s = String::from("hello, world");
    // convert String to Vec
    // 将 String 转换成 Vec 类型
    // as_bytes() into_bytes()
    let _s = s.as_bytes();
    s
}

fn print_str(s: &String) {
    println!("{s}")
}
