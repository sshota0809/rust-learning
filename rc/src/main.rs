use std::cell::{Ref, RefCell};
use std::rc::Rc;

#[derive(Debug)]
struct Test {
    text: Option<Rc<String>>,
}

#[derive(Debug)]
struct Test2 {
    text: Option<Rc<RefCell<String>>>,
}

#[derive(Debug)]
struct Test3 {
    test4: Test4,
}

#[derive(Debug)]
struct Test4 {
    text: String,
}

fn setText(test: &mut Test) {
    let s = Rc::new(String::from("this is test"));
    test.text = Some(Rc::clone(&s));
}

fn setText2(test2: &mut Test2) {
    let s = Rc::new(RefCell::new(String::from("this is test")));
    test2.text = Some(Rc::clone(&s));
}

fn main() {
    let mut test = Test { text: None };
    setText(&mut test);
    println!("{}", test.text.as_ref().unwrap());

    let mut test2 = Test2 { text: None };
    setText2(&mut test2);
    *test2.text.as_ref().unwrap().borrow_mut() = String::from("this is test2");
    println!("{}", test2.text.as_ref().unwrap().borrow());

    let test3 = Rc::new(RefCell::new(Test3 {
        test4: Test4 {
            text: String::from("this is test3"),
        },
    }));

    let test3_2 = Rc::clone(&test3);
    println!("{}", test3_2.borrow().test4.text);

    (*test3_2.borrow_mut()).test4.text = String::from("this is test3_2");
    println!("{}", test3_2.borrow().test4.text);
}
