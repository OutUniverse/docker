use std::fmt;

#[derive(Debug)]
struct Person {
    name: String,
    age: u8
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Person name is {}, age is {}", self.name, self.age)
    }
}

struct Array(Vec<i32>);

impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Array is {:?}", self.0)
    }
}

#[allow(unused)]
fn main() {
    println!("{a}", a = 1);

    eprintln!("error println test");

    let i = 3.1415926;
    let s = String::from("hello");
    let v = Array(vec![1, 2, 3]);
    let p = Person {
        name: "sunface".to_string(),
        age: 18,
    };

    println!("{}, {}, {}, {}", i, s, p, v);

    println!("{1} {} {1} {} {0} {} {2}", 1, 2, 3);

    println!("{} {value}", 1, value = 2);

    str_width();

    num_width();

    personalized_fill_char();

    precision();

    system();

    exponent();

    point_addr();

    escape();
} 

fn format_variable() {
    fn get_format() -> (i32, i32) {
        (5, 2)
    }

    fn get_scores() -> Vec<(String, i32)> {
        vec![("a".to_string(), 1), ("b".to_string(), 2)]
    }

    let (width, precision) = get_format();

    for (name, score) in get_scores() {
        format!("{name}: {score:width$.precision$}");
    }
}

fn escape() {
    // { 用 {{转义， "用\"

    println!("{{Hellow world}}, \"lalala\"");
}

fn point_addr() {
    let v = vec![1, 2, 3];

    println!("{:p}", v.as_ptr());
}

fn exponent() {
    let a = 10000000;

    println!("{:2e}", a);
    println!("{:2E}", a);
}

fn system() {
    let a = 55;
    // 二进制, #b
    println!("{:#b}", a);
    // 八进制, #o
    println!("{:#o}", a);
    // 十进制
    println!("{}", a);
    // 小写十六进制, #x
    println!("{:#x}", a);
    // 大写十六进制, #X
    println!("{:#X}", a);
    // 不带前缀的十六进制x
    println!("{:x}", a);
    // 不带前缀的十六进制X
    println!("{:X}", a);
    // 使用0填充二进制，宽度为10
    println!("{:010b}", a);
}

fn precision() {
    let a = 3.1415926;

    println!("{:.2}", a);
    // 指定加号
    println!("{:+.2}", a);
    // 不保留小树
    println!("{:.0}", a);
    // 通过参数指定精度
    println!("{:.1$}", a, 3);
    // 保留字符串前N位
    println!("{:.1$}", "啦啦啦，我是卖报的小画家", 5);
    // {:.*}接受俩个参数，第一个是精度，第二个是被格式化的值
    println!("{:.*}", 4, "world");
}

fn str_width() {
    // 字符串默认使用空格进行填充，是左对齐

    // 使用5作为宽度
    println!("{:5} finish", "x");
    // 使用第一个参数作为宽度，第二个参数为展示值
    println!("{1:0$} finish", 10, "x");
    // 指定值作为宽度和值
    println!("{value:width$} finish", width = 5, value = "x");
    // 在最后在显示个5
    println!("{:1$}{}", "x", 5);
}

fn num_width() {
    // 数字默认用空格进行填充，是右对齐
    println!("start {:5}", 5);
    // 输出+
    println!("start {:+}", 5);
    // 使用0进行填充
    println!("start {:01$}", 5, 5);
    // 负号也要站一位宽度
    println!("start {:5}", -5);
}

fn personalized_fill_char() {
    // 使用指定符号填充，必须制指定对齐方式
    println!("start {:&>5}", 1);
    println!("start {:&>1$}", "x", 8);
    println!("{1:x<0$} end", 5, 1);
    println!("start {1:1^0$} end", 10, "x");
}
