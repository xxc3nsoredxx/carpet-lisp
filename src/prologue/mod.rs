pub mod arithmetic;
pub mod io;

use std::rc::Rc;

use crate::object::NativeArgs;
use crate::{Error, Object, Type};

pub fn cons(mut args: Box<NativeArgs>) -> Result<Rc<Object>, Error> {
    if args.len() != 2 {
        return Err(Error::Parameters);
    }

    let car = args.next().unwrap();
    let cdr = args.next().unwrap();

    Ok(Rc::new(Object::Cons(Rc::clone(&car), Rc::clone(&cdr))))
}

pub fn car(mut args: Box<NativeArgs>) -> Result<Rc<Object>, Error> {
    if args.len() != 1 {
        return Err(Error::Parameters);
    }

    let cons = args.next().ok_or(Error::Parameters)?;

    cons.car()
        .ok_or(Error::Type(Type::Cons, Type::from(&*cons)))
}

pub fn cdr(mut args: Box<NativeArgs>) -> Result<Rc<Object>, Error> {
    if args.len() != 1 {
        return Err(Error::Parameters);
    }

    let cons = args.next().ok_or(Error::Parameters)?;

    let None = args.next() else {
        return Err(Error::Parameters);
    };

    cons.cdr()
        .ok_or(Error::Type(Type::Cons, Type::from(&*cons)))
}

pub fn equal(mut args: Box<NativeArgs>) -> Result<Rc<Object>, Error> {
    if match args.next() {
        Some(object) => args.all(|o| *o == *object),
        None => return Err(Error::Parameters),
    } {
        Ok(Rc::new(Object::True))
    } else {
        Ok(Rc::new(Object::Nil))
    }
}

pub fn list(mut args: Box<NativeArgs>) -> Result<Rc<Object>, Error> {
    match args.next() {
        Some(object) => Ok(Rc::new(Object::Cons(object, list(args)?))),
        None => Ok(Rc::new(Object::Nil)),
    }
}

pub fn is_nil(mut args: Box<NativeArgs>) -> Result<Rc<Object>, Error> {
    if args.len() != 1 {
        return Err(Error::Parameters);
    }

    match &*args.next().unwrap() {
        Object::Nil => Ok(Rc::new(Object::True)),
        _ => Ok(Rc::new(Object::Nil)),
    }
}

pub fn push_back(mut args: Box<NativeArgs>) -> Result<Rc<Object>, Error> {
    if args.len() < 1 {
        return Err(Error::Parameters);
    }

    let mut elements = Vec::new();

    let first = match args.next().unwrap() {
        object if object.is_cons() || object.is_nil() => object.iter_cars().unwrap(),
        object => return Err(Error::Type(Type::Cons, Type::from(&*object))),
    };

    elements.extend(first);
    elements.extend(args);

    list(Box::new(elements.into_iter()))
}