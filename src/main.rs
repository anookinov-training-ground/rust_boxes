use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

// Box implements Deref and Drop traits
enum List {
    Cons(i32, Box<List>),
    Nil,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

use crate::List::{Cons, Nil};
use crate::RcList::{RcCons, RcNil};
use crate::RcRefCellList::{RcRefCellCons, RcRefCellNil};
use std::cell::RefCell;
use std::rc::Rc;

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

#[derive(Debug)]
enum RcList {
    RcCons(i32, Rc<RcList>),
    RcNil,
}

#[derive(Debug)]
enum RcRefCellList {
    RcRefCellCons(Rc<RefCell<i32>>, Rc<RcRefCellList>), // Cell<T> is similar to RefCell<T> except the value is copied instead of giving references to the inner value
    RcRefCellNil,
}

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *(y.deref())); // equal to assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m); // &MyBox<String> -> &String -> &str
    hello(&(*m)[..]); // if Rust didn't implement deref coercion (Deref::deref is called multiple times)

    // Rust deref coercion cases:
    // 1) From &T to &U when T: Deref<Target=U>
    // 2) From &mut T to &mut U when T: DerefMut<Target=U>
    // 3) From &mut T to &U when T: Deref<Target=U> | reverse is not possible

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
    // c.drop(); // error
    drop(c); // call std::mem::drop function to drop early
    println!("CustomSmartPointer dropped before the end of main.");

    let a = Rc::new(RcCons(5, Rc::new(RcCons(10, Rc::new(RcNil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = RcCons(3, Rc::clone(&a)); // can also use a.clone() but it will perform deep copies
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = RcCons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
        println!("{:#?}", c);
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    println!("{:#?}", b);

    // let x = 5;
    // let y = &mut x; // cannot borrow immutable value mutably
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(RcRefCellCons(Rc::clone(&value), Rc::new(RcRefCellNil)));

    let b = RcRefCellCons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = RcRefCellCons(Rc::new(RefCell::new(4)), Rc::clone(&a));
    
    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
