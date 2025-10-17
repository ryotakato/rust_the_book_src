
fn main() {

    mybox();

    cons();

    node();
}

use std::ops::Deref;
use std::fmt::Display;

struct MyBox<T: Display>(T);

impl<T: Display> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T: Display> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T: Display> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Drop trait: {}", &self.0);
    }
}



fn mybox() {

    println!("-- mybox ----------------------------");

    let x = 5;
    let y = MyBox::new(x);

    //assert_eq!(5, x);
    //assert_eq!(5, *y);
    println!("{}", x);
    println!("{}", *y);
    println!("{}", *y.deref());
    std::mem::drop(y);

    println!("y is already dropped here");

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}



use std::rc::{Rc, Weak};
use std::cell::RefCell;




#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[derive(Debug)]
enum RcList {
    RcCons(i32, Rc<RcList>),
    RcNil,
}

#[derive(Debug)]
enum RefRcList {
    RefRcCons(Rc<RefCell<i32>>, Rc<RefRcList>),
    RefRcNil,
}

#[derive(Debug)]
enum CycleRefList {
    CycleRefCons(i32, RefCell<Rc<CycleRefList>>),
    CycleRefNil,
}

impl CycleRefList {
    fn tail(&self) -> Option<&RefCell<Rc<CycleRefList>>> {
        match *self {
            CycleRefCons(_, ref item) => Some(item),
            CycleRefNil => None,
        }
    }
}


use List::{Cons, Nil};
use RcList::{RcCons, RcNil};
use RefRcList::{RefRcCons, RefRcNil};
use CycleRefList::{CycleRefCons, CycleRefNil};

fn cons() {

    println!("-- cons list pattern 1 ----------------------------");

    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3, 
                Box::new(Nil))))));


    println!("{:?}", list);

    println!("-- cons list pattern 2 ----------------------------");

    let a = Rc::new(RcCons(5, Rc::new(RcCons(10, Rc::new(RcNil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = RcCons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = RcCons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));


    println!("-- cons list pattern 3 ----------------------------");

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(RefRcCons(Rc::clone(&value), Rc::new(RefRcNil)));

    let b = RefRcCons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = RefRcCons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);



    println!("-- cons list pattern 4 ----------------------------");


    let a = Rc::new(CycleRefCons(5, RefCell::new(Rc::new(CycleRefNil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(CycleRefCons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // cycle, so infinite, as a result, stackoverflow
    // println!("a next item = {:?}", a.tail());
}


#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>,
}


fn node() {

    println!("-- node ----------------------------");

    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf),);

    {

        let branch = Rc::new(Node {
            value: 5,
            children: RefCell::new(vec![Rc::clone(&leaf)]),
            parent: RefCell::new(Weak::new()),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);


        println!("branch strong = {}, weak = {}", Rc::strong_count(&branch), Rc::weak_count(&branch),);
        println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf),);

    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf),);
}



