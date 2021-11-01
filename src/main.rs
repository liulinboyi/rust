// 创建变量关键字let rust中变量，默认是不可变的
// 常量使用const定义 可以在任何作用域中声明
// 常量和变量最大的区别是 常量是在编译期间求值的 而不可变变量通常是在运行期间求值的
// 隐藏 可以声明和前面变量同名的新变量
//

pub mod average;
pub mod int_overflow;
pub mod mutuple;
pub mod array;
pub mod my_slice;

const COUNT: i32 = 10;

fn get_number() -> i32 {
    42
}

fn main() {
    test_main();
    test_avg();
    test_int_over_flow();
    test_mutuple();
    test_array();
    test_slice();
}

fn test_main() {
    // let x = 6; // 默认不可变
    let mut x = 10; // 添加mut可变
                    // 宏是一个支持可变参数的函数
    println!("The value of x is {}", x); // 这是一个宏，宏与函数最大的区别是，宏调用需要在后面加上感叹号

    x = 6;

    let y = get_number(); // 不可变变量通常是运行期间求值
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
    print!("The value of COUNT is {}", COUNT);

    // 隐藏
    let a = 1;
    let a = a + 1;
    let a = a * 2;
    // 推荐这种写法
    // 隐藏这种操作，在大多数，不可变变量的语言中很常见
    // 新变量和前一个变量名称相同时，新变量会隐藏掉前面的变量
    println!("The value of a is {}", a);
}

fn test_avg() {
    assert_eq!(average::avg(4294967295, 4294967295), 4294967295);
    assert_eq!(average::avg(0, 0), 0);
    assert_eq!(average::avg(10, 20), 15);
    assert_eq!(average::avg(4294967295, 1), 2147483648);
    println!("passed");
}

fn test_int_over_flow() {
    int_overflow::int_over_flow()
}

fn test_mutuple() {
    mutuple::mutuple()
}

fn test_array() {
    array::array();
}

fn test_slice() {
    my_slice::slice();
}
