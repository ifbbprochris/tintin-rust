fn main() {
    let s1 = 10u32;
    let s2 = s1;
    let s3 = s2;
    println!("s1: {s1}");
    println!("s2: {s2}");
    println!("s3: {s3}");

    let str1 = String::from("this is str1");
    let str2 = str1;
    // println!("str1: {str1}"); 这里str1已经没有所有权了， 所以不能使用str1打印
    println!("str2: {str2}");

    // 调用一个不可变例子
    // nonchange(&str2); 这里变异会报错， 因为 str2 声明的时候就是一个不可变的，所以引用也一样，不允许修改引用的值

    // 调用一个可变引用例子
    let mut str3 = String::from("hello");
    change(&mut str3);
    println!("after call change str3: {str3}");
}

fn nonchange(some_string: &mut String) {
    some_string.push_str(", after nonchange");
}

fn change(some_string: &mut String) {
    some_string.push_str(", after change");
}
