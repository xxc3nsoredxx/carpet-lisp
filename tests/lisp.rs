use lisp::{Interpreter, Object, Reader};
use std::rc::Rc;

fn eval(expr: &str) -> Rc<Object> {
    let mut interpreter = Interpreter::new();
    let reader = Reader::new(expr);
    let mut obj = None;
    for read in reader {
        obj = Some(
            interpreter
                .eval(Rc::new(Object::from(read.unwrap())))
                .unwrap(),
        );
    }
    obj.unwrap()
}

#[test]
fn test_lambda() {
    assert!(matches!(
        *eval("(lambda (a b c) (+ a b c))"),
        Object::Function(..)
    ))
}

#[test]
fn test_self_evaluatiing() {
    assert!(matches!(*eval("1"), Object::Int(1)));
}

#[test]
fn test_def() {
    assert!(matches!(*eval("(def x 1) x"), Object::Int(1)));
}

#[test]
fn test_set() {
    assert!(matches!(*eval("(def x 1) (set x 2) x"), Object::Int(2)));
}
