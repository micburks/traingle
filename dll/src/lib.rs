
use std::rc::{Rc,Weak};
use std::cell::RefCell;

#[derive(Debug)]
pub struct DLL<T> {
    head: Option<Rc<RefCell<Item<T>>>>,
    tail: Option<Rc<RefCell<Item<T>>>>,
    len: usize,
}

impl<T> DLL<T> {
    pub fn new(values: Vec<T>) -> DLL<T> {
        if values.len() == 0 {
            return DLL {
                head: None,
                tail: None,
                len: 0,
            }
        }
        let mut len = 0;
        let mut items: Vec<Rc<RefCell<Item<T>>>> = vec![];
        let mut previous: Option<Rc<RefCell<Item<T>>>> = None;
        for value in values {
            let item = Rc::new(RefCell::new(Item::new(value)));
            if let Some(prev) = &previous {
                prev.borrow_mut().next = Some(Rc::downgrade(&item));
            }
            item.borrow_mut().prev = Some(Rc::downgrade(&previous.unwrap()));
            previous = Some(Rc::clone(&item));
            items.push(item);
            len += 1;
        }
        DLL {
            head: Some(Rc::clone(&items[0])),
            tail: Some(Rc::clone(&items[items.len() - 1])),
            len,
        }
    }
    pub fn head(&self) -> Option<Rc<RefCell<Item<T>>>> {
        match &self.head {
            Some(r) => Some(Rc::clone(&r)),
            None => None,
        }
    }
    pub fn tail(&self) -> Option<Rc<RefCell<Item<T>>>> {
        match &self.tail {
            Some(r) => Some(Rc::clone(&r)),
            None => None,
        }
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn get(&self, index: usize) -> Option<Rc<RefCell<Item<T>>>> {
        if index > self.len {
            return None;
        }
        let mut i = 0;
        let mut reference = match &self.head {
            Some(r) => Rc::clone(&r),
            None => return None,
        };
        while i < index && i < self.len {
            reference = {
                let reference_borrowed = reference;
                let next_borrowed = &reference_borrowed.borrow().next;
                if let Some(n) = next_borrowed {
                    Rc::clone(&n.upgrade().unwrap())
                } else {
                    return None;
                }
            };
            if i == index {
                break;
            }
            i += 1;
        }
        Some(reference)
    }
}

#[derive(Debug)]
pub struct Item<T> {
    pub value: T,
    next: Option<Weak<RefCell<Item<T>>>>,
    prev: Option<Weak<RefCell<Item<T>>>>,
}

impl<T> Item<T> {
    fn new(value: T) -> Item<T> {
        Item {
            value,
            next: None,
            prev: None,
        }
    }
    pub fn next(&self) -> Option<Rc<RefCell<Item<T>>>> {
        match &self.next {
            Some(r) => Some(r.upgrade().unwrap()),
            None => None,
        }
    }
    pub fn prev(&self) -> Option<Rc<RefCell<Item<T>>>> {
        match &self.prev {
            Some(r) => Some(r.upgrade().unwrap()),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accessors_work() {
        let dll = DLL::new(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        // assert_eq!(dll.head().unwrap().borrow().value, 0);
        // assert_eq!(dll.get(1).unwrap().borrow().value, 1);
        // assert_eq!(dll.tail().unwrap().borrow().value, 9);
        // assert_eq!(dll.tail().unwrap().borrow().prev().unwrap().borrow().value, 8);
        // assert_eq!(dll.get(1).unwrap().borrow().next().unwrap().borrow().value, 2);
    }

    #[test]
    #[ignore]
    fn deleting_works() {
        /*
        let dll = DLL::new(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(dll.len(), 10);

        dll.remove(9);
        assert_eq!(dll.tail(), Some(8));
        assert_eq!(dll.len(), 9);

        dll.remove(0);
        assert_eq!(dll.head(), Some(1));
        assert_eq!(dll.len(), 8);
        */
    }
}
