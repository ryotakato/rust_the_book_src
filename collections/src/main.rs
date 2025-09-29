use std::fmt::{self, Display, Formatter };
use std::collections::HashMap;
use std::io;

fn main() {

    calc_vector();


    pig_latin();


    manage_employees();


}


fn manage_employees() {

    println!("---------------------------------");

    let mut employees:HashMap<String, Vec<String>> = HashMap::new();

    loop {

        println!("Please input command (show, add, quit)");

        let mut r_command = String::new();

        io::stdin()
            .read_line(&mut r_command)
            .expect("Failed to read line");

        let command: Vec<&str> = r_command.trim().split(' ').collect();

        //println!("{:?}", command);

        match command[0] {
            "show" => println!("{:?}", employees),
            "add" => {
                employees.entry(command[3].to_string())
                    .or_insert(Vec::new())
                    .push(command[1].to_string());
                println!("done!")
            },
            _ => break,
        }
    }
}


fn pig_latin() {
    println!("---------------------------------");


    // NZ anthem
    let origin: Vec<&str> = vec!["God", "of", "Nations", "at", "thy", "feet", "In", "the", "bonds", "of", "love", "we", "meet"];

    let mut pig_latin: Vec<String> = Vec::new();
    for s in origin {
        let i = s.char_indices().nth(1).unwrap().0;
        let s_lower = &s.to_lowercase();
        pig_latin.push(match &s_lower[..i] {
            "a"|"i"|"u"|"e"|"o" => format!("{}-hay", &s),
            _ => format!("{}-{}ay", &s[i..], &s[..i]),
        });
    }

    println!("{:?}", pig_latin);





}


fn calc_vector() {
    println!("---------------------------------");


    let mut v: Vec<i32> = vec![41,52,73,2,77,21,83,23,92,16,44,39,65];
    v.push(75);
    v.push(44);
    v.push(50);
    v.push(50);
    v.push(50);

    let mean = mean(&v);
    println!("mean: {}", mean);
    let median = median(&mut v);
    println!("median: {}", median);
    let mode = mode(&v);
    println!("mode: {}", mode);

}


fn mean(v: &Vec<i32>) -> f32 {
    (v.iter().sum::<i32>() as f32)/ (v.len() as f32)
}

fn median(v: &mut Vec<i32>) -> f32 {
    v.sort();
    //println!("{:?}", v);
    let mid = v.len() / 2;
    if mid % 2 == 0 {
        (v[mid - 1] + v[mid + 1]) as f32 / 2.0
    } else {
        v[mid] as f32
    }
}

fn mode(v: &Vec<i32>) -> i32 {
    let mut hash = HashMap::new();
    for i in v {
        let count = hash.entry(i).or_insert(0);
        *count += 1
    }
    //println!("{:?}", hash);

    let mut max = 0;
    let mut max_value = 0;
    for (key, value) in &hash {
        if *value > max {
            max = *value;
            max_value = **key;
        }
    }
    max_value
}
