fn main() {
    let s = "abc";
    for c in s.chars() {
        print!("{}", c);
    }
    println!();

    let n1 = [0, 1, 2];
    for &i in n1.iter() {
        //&i模式匹配解构引用，i此时是个准确的值
        print!("{}", i);
    }
    println!();

    let n2 = [0, 1, 2];
    for i in n2.iter() {
        //i此时是个引用，避免拷贝
        print!("{}", i); //按理来说应该用*i解引用，但是print!()会自动解引用
    }
    println!();

    let mut a1 = [String::from("0"), String::from("1")];
    for i in a1.iter_mut() {
        //i此时是个引用，避免拷贝
        i.push_str("!");
    }
    println!("{:?}", a1);
}
