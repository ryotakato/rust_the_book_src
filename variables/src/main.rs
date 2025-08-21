fn main() {

    // mut
    println!();
    println!("-- mut -----------------");
    let mut x = 5;
    println!("This value of x is : {}", x);
    x = 6;
    println!("This value of x is : {}", x);
    
    // shadowing
    println!();
    println!("-- shadowing -----------------");
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x in the inner scope is: {}", x);
    
    // variable
    println!();
    println!("-- variable -----------------");
    let a: i32 = 345;
    println!("number is {}", a);
    let t = true;
    let f: bool = false;
    

    // function & scope
    println!();
    println!("-- function & scope -----------------");
    let x = six();
    println!("The value of x is: {}", x);
    let x = recur(0, 10);
    println!("The value of x is: {}", x);
    

    // calc degree
    println!();
    println!("-- degree -----------------");
    let c:f32 = 20.4;
    let f = calc_c_to_f(c);
    println!("c is {}, f is {}", c, f);
    let c_rev = calc_f_to_c(f);
    println!("c is {}, f is {}, c_rev is {}", c, f, c_rev);

    // fib recur
    println!();
    println!("-- fib -----------------");
    println!("The value of x is: {}", fib_recur(0,1,0));
    println!("The value of x is: {}", fib_recur(0,1,1));
    println!("The value of x is: {}", fib_recur(0,1,2));
    println!("The value of x is: {}", fib_recur(0,1,3));
    println!("The value of x is: {}", fib_recur(0,1,4));
    println!("The value of x is: {}", fib_recur(0,1,5));
    println!("The value of x is: {}", fib_recur(0,1,6));
    println!("The value of x is: {}", fib_recur(0,1,22));

    // The 12 Days of Christmas Lyrics
    println!();
    println!("-- The 12 Days of Christmas Lyrics -----------------");
    let base = ("On the twelfth day of Christmas,", "my true love sent to me", "a partridge in a pear tree.");
    let list = [
        "",
        "Two turtle doves,",
        "Three French hens,",
        "Four calling birds,",
        "Five gold rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,",
    ];

    for (idx, sentense) in list.iter().enumerate() {
        println!("{}", base.0);
        println!("{}", base.1);

        let last_sentense = base.2;
        let last_sentense = (&last_sentense[..1], &last_sentense[1..]);
        if idx == 0 {
            println!("{}{}", last_sentense.0.to_uppercase(), last_sentense.1);
        } else {
            for sentense in (&list[1..idx+1]).into_iter().rev() {
                println!("{}", sentense);
            }
            println!("{}{}{}", if idx == 0 { "" } else { "And " } , last_sentense.0, last_sentense.1);
        }
        println!();
    }


}

fn fib_recur(i1: i32, i2: i32, c: i32) -> i32 {
    if c == 0 {
        i1
    } else if c == 1 {
        i2
    } else {
        fib_recur(i2, i1 + i2, c - 1)
    }
}

fn calc_c_to_f(c: f32) -> f32 {
    1.8 * c + 32.0
}

fn calc_f_to_c(f: f32) -> f32 {
    (f - 32.0) / 1.8
}

fn six() -> i32 {
    1 + {
        1 + {
            1 + {
                1 +  {
                    1 + {
                        1
                    }
                }
            }
        }
    }
}

fn recur(t: i32, c: i32) -> i32 {
    if c == 0 {
        t
    } else {
        recur(t + 1, c - 1)
    }
}
