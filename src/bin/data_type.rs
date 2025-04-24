use std::num::FpCategory::{Infinite, Nan};

fn main() {
    // scalar();
    compound();
    // str();
}

fn scalar() {
    const C1: i32 = 1;
    const C2: i32 = 2;
    const C3: i32 = C1 + C2;
    println!("{}", C3);

    let a1: i8 = 1; //有符号，-2^-7到2^7-1
    let a2: u8 = 2; //无符号，0到2^8-1
    println!("{}", a1 + a2 as i8); //必须同类型才能进行操作
    println!(
        "{:?} {:?}",
        a1.overflowing_add(126),
        a1.overflowing_add(127)
    ); //检测是否溢出
    let a3: f64 = 3.4028236; //有符号
    let a4: usize = 3; //无符号，系统位数
    println!("{} {} {:?} {:?}", a3, a4, Nan, Infinite);

    let a5: u8 = b'a'; //1字节
    let a6 = 'a'; //4字节
    let a7 = true; //为了对齐内存，通常会被存储为1字节
    println!("{} {} {}", a5, a6, a7);
}

fn compound() {
    //固定长度，异构，分配在栈
    let tup = (0, 'a', "ab", true);
    println!("{:?} {:?}", tup, tup.0);

    //固定长度，同构，分配在栈
    let arr1 = [0, 1, 2, 3, 4];
    println!("{:?} {:?}", arr1[0], &arr1[0..2]); //切片的本质是借用
    let arr2 = ["arr"; 3]; //重复值初始化
    println!("{:?}", arr2);

    //动态长度，同构，分配在堆
    let vec1 = vec![1, 2, 3]; //直接初始化
    let vec2 = vec!["vec"; 3]; //重复值初始化
    println!("{:?} {:?}", vec1, vec2);
    let mut vec3 = Vec::new(); //构造函数
    vec3.push(0);
    let mut vec4 = Vec::with_capacity(5); //预分配容量，防止频繁扩容
    vec4.push(0);
    println!("{:?} {:?}", vec3, vec4);
    let vec5 = (0..5).collect::<Vec<_>>(); //从迭代器收集
    let vec6 = "hello".chars().collect::<Vec<_>>();
    println!("{:?} {:?}", vec5, vec6);
}

fn str() {
    let mut s1 = String::from("Hello");
    s1.push_str(", ");
    let s2 = "world!"; //静态的，是一个指向某个字符串数据的引用
    let s3 = s1 + s2; //s1的所有权转移到s3中，后续再使用s1会报错
    println!("{}", s3);

    let s4 = b"bytestream";
    println!("{:?}", s4);
    println!("{}", String::from_utf8_lossy(s4));
    println!("{:?}", String::from_utf8(Vec::from(s4)));
}
