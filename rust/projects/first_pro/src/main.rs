use std::str;

fn main() {

    test_byte_string();

}

fn test_byte_string() {
    // 注意，这并不是 `&str` 类型了！
    let bytestring: &[u8; 21] = b"this is a byte string";


    // 字节数组没有实现 `Display` 特征，因此只能使用 `Debug` 的方式去打印
    println!("A byte string: {:?}", bytestring);

    // 字节数组也可以使用转义
    let escaped = b"\x52\x75\x73\x74 as bytes";
    // ...但是不支持 unicode 转义
    // let escaped = b"\u{211D} is not allowed";
    println!("Some escaped bytes: {:?}", escaped);


    // raw string
    let raw_bytestring = br"\u{211D} is not escaped here";
    println!("{:?}", raw_bytestring);

    // 将字节数组转成 `str` 类型可能会失败
    if let Ok(my_str) = str::from_utf8(raw_bytestring) {
        println!("And the same as text: '{}'", my_str);
    }

    let _quotes = br#"You can also use "fancier" formatting, \
                    like with normal raw strings"#;

    // 字节数组可以不是 UTF-8 格式
    let shift_jis = b"\x82\xe6\x82\xa8\x82\xb1\x82\xbb"; // "ようこそ" in SHIFT-JIS

    // 但是它们未必能转换成 `str` 类型
    match str::from_utf8(shift_jis) {
        Ok(my_str) => println!("Conversion successful: '{}'", my_str),
        Err(e) => println!("Conversion failed: {:?}", e),
    };
}

#[cfg(test)]
fn utf8_slice_test() {
    let s = "中国哪部电影好看？";

    let s1 = utf8_slice::slice(s, 0, 2);

    let s2 = utf8_slice::from(s, 4);

    let s3 = utf8_slice::till(s, 2);

    let len = utf8_slice::len(s);

    println!("{}", s);
    println!("{}", s1);
    println!("{}", s2);
    println!("{}", s3);
    println!("{}", len);
}

#[cfg(test)]
fn handle_utf8_str() {
    for a in "中国".chars() {
        println!("{}", a);
    }

    for a in "中国".bytes() {
        println!("{}", a);
    }
}

#[cfg(test)]
fn keep_origin_format() {
    println!("{}", "hello \\x52\\x75\\x73\\x74");
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    // 如果字符串包含双引号，可以在开头和结尾加 #
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // 如果还是有歧义，可以继续增加，没有限制
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);
}
 
#[cfg(test)]
fn byte_escape() {
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );

    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here ->\
                        <- can be escaped too!";
    println!("{}", long_string);
}

#[cfg(test)]
fn format_test() {
    let s1 = "hello";
    let s2 = String::from("world");

    let s = format!("{} {}", s1, s2);

    println!("{} {} {}", s, s1, s2);
}

#[cfg(test)]
fn add_test() {
    let s1 = String::from("Hello");

    let s1 = s1 + " World";

    println!("{}", s1);
}

#[cfg(test)]
fn clear_test() {
    let mut s = String::from("你好世界！");

    s.clear();

    dbg!(s);
}

#[cfg(test)]
fn truncate_test() {
    let mut s = String::from("你好啊啊啊");

    s.truncate(3);

    dbg!(s);
}

#[cfg(test)]
fn remove_test() {
    let mut s = String::from("你好世界！");

    let s1 = s.remove(0);

    dbg!(s);
    dbg!(s1);
}

#[cfg(test)]
fn pop_test() {
    let mut s = String::from("你好，世界!");
    let p1 = s.pop();
    let p2 = s.pop();

    dbg!(p1);
    dbg!(p2);
    dbg!(s);
}

#[cfg(test)]
fn replace_range_test() {
    let mut s = "Hello, rust".to_string();

    s.replace_range(7..8, "R");

    println!("{}", s);
}

#[cfg(test)]
fn  replacen_test () {
    let s = String::from("Hello, rust, rust!");

    let s1 = s.replacen("rust", "RUST", 1);

    println!("{}", s1);

    let s2 = "Hello, rust, rust, rust!";

    let s3 = s2.replacen("rust", "RUST", 2);

    println!("{}", s3);
}

#[cfg(test)]
fn replace_test() {
    let s = String::from("Hello, rust!");

    let s1 = s.replace("rust", "RUST");

    println!("{}", s1);

    let s2 = "Hello, rust!";
    let s3 = s2.replace("rust", "RUST");

    println!("{}", s3);
}

#[cfg(test)]
fn push_test() {
    let mut s = String::from("Hello ");

    s.push('r');

    println!("{}", s);

    s.push_str("ust");

    println!("{}", s);
}

#[cfg(test)]
fn insert_test() {
    let mut s = String::from("Hello ");

    s.insert(5, ',');

    println!("{}", s);

    s.insert_str(7, "world");

    println!("{}", s);
}
