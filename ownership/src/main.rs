use std::time::Instant;

fn main() {

    //// difference between copy and move
    //let x = 5;
    //let y = x;
    //println!("x = {}, y = {}", x, y);

    //let s1 = String::from("hello");
    ////let s2 = s1; // compile error
    //let s2 = s1.clone();
    //println!("s1 = {}, s2 = {}", s1, s2);

    const COUNT: usize = 100_000;
    //let list = ["Rust"; COUNT];
    static LIST: [&'static str; COUNT] = ["\tRust\t"; COUNT];
    let now = Instant::now();

    // push_str
    let mut s_res = String::from("");
    for _i in 0..COUNT {
        s_res.push_str("Rust");
    }
    //println!("{}", s_res);
    println!("push_str: {:?}", Instant::now().duration_since(now));
    let now = Instant::now();

    // +
    let mut s_res = String::from("");
    for _i in 0..COUNT {
        s_res = s_res + "Rust";
    }
    //println!("{}", s_res);
    println!("+: {:?}", Instant::now().duration_since(now));
    let now = Instant::now();

    // +=
    let mut s_res = String::from("");
    for _i in 0..COUNT {
        s_res += "Rust";
    }
    //println!("{}", s_res);
    println!("+=: {:?}", Instant::now().duration_since(now));
    let now = Instant::now();

    // format!
    let mut s_res = String::from("");
    for i in 0..COUNT {
        s_res = format!("{}Rust", s_res);
    }
    //println!("{}", s_res);
    println!("format!: {:?}", Instant::now().duration_since(now));
    let now = Instant::now();


    // join
    let s_res = LIST.iter().map(|x| x.trim()).collect::<Vec<_>>().join("");
    //println!("{}", s_res);
    println!("join: {:?}", Instant::now().duration_since(now));



}
