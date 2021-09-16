
#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

// Current limitation is that Cons list must be a linear list and cannot be branched

impl<T> List<T> {
    fn new(val: T) -> List<T> {
        List::Cons(val, Box::new(List::Nil))
    }
    fn cons(val: T, list: List<T>) -> List<T> {
        List::Cons(val, Box::new(list))
    }
    fn car(list: &List<T>) -> Result<&T, &'static str> {
        match list {
            List::Cons(x, _) => Ok(x),
            List::Nil => Err("Cannot get car of Nil"),
        }
    }
    fn cdr(list: &List<T>) -> Result<&List<T>, &'static str> {
        match list {
            List::Cons(_, y) => Ok(y),
            List::Nil => Err("Cannot get cdr of Nil"),
        }
    }
    fn cadr(list: &List<T>) -> Result<&T, &'static str> {
        List::car(List::cdr(list).unwrap())
    }
}

fn main() {
    let x = 13;
    let y = 14;
    // let a = List::Cons(x, Box::new(List::Nil));
    // let a = List::cons(x, List::Nil);
    // let a = List::new(x);
    let a = List::cons(x, List::new(y));
    println!("{:?}", a);
    println!("{:?}", List::car(&a).unwrap());
    println!("{:?}", List::cdr(&a).unwrap());
    // println!("{:?}", List::car(List::cdr(&a).unwrap()).unwrap());
    println!("{:?}", List::cadr(&a).unwrap());
}
