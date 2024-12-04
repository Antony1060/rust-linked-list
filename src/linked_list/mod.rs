use std::cell::RefCell;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;
use std::rc::Rc;

type LinkedListEntry<T> = Rc<LinkedListNode<T>>;

#[allow(dead_code)]
pub struct LinkedListNode<T> {
    pub value: T,
    prev: RefCell<Option<LinkedListEntry<T>>>,
    next: RefCell<Option<LinkedListEntry<T>>>,
}

impl<T> Deref for LinkedListNode<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> Display for LinkedListNode<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.value)
    }
}

impl<T> Debug for LinkedListNode<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self.value)
    }
}

pub struct LinkedListIterator<T> {
    first: bool,
    curr: Option<LinkedListEntry<T>>,
}

#[allow(dead_code)]
pub struct LinkedList<T> {
    head: RefCell<Option<LinkedListEntry<T>>>,
    tail: RefCell<Option<LinkedListEntry<T>>>,
    pub size: usize,
}

#[allow(dead_code)]
impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            size: 0,
            head: RefCell::from(None),
            tail: RefCell::from(None),
        }
    }

    pub fn new_with(init: T) -> LinkedList<T> {
        let head: LinkedListEntry<T> = Rc::new(LinkedListNode {
            value: init,
            next: RefCell::from(None),
            prev: RefCell::from(None),
        });

        LinkedList {
            size: 1,
            head: RefCell::from(Some(Rc::clone(&head))),
            tail: RefCell::from(Some(head)),
        }
    }

    fn init_with(&mut self, item: LinkedListEntry<T>) {
        self.head.replace(Some(Rc::clone(&item)));
        self.tail.replace(Some(item));

        self.size = 1;
    }

    pub fn push_back(&mut self, item: T) {
        let item = Rc::new(LinkedListNode {
            value: item,
            next: RefCell::from(None),
            prev: RefCell::from(None),
        });

        if self.size == 0 {
            self.init_with(Rc::clone(&item));
            return;
        }

        // will exist, size isn't 0
        let tail = self.tail.replace(Some(Rc::clone(&item))).unwrap();

        item.prev.replace(Some(Rc::clone(&tail)));

        tail.next.replace(Some(Rc::clone(&item)));

        self.size += 1;
    }

    pub fn push_front(&mut self, item: T) {
        let item = Rc::new(LinkedListNode {
            value: item,
            next: RefCell::from(None),
            prev: RefCell::from(None),
        });

        if self.size == 0 {
            self.init_with(Rc::clone(&item));
            return;
        }

        // will exist, size isn't 0
        let head = self.head.replace(Some(Rc::clone(&item))).unwrap();
        item.next.replace(Some(Rc::clone(&head)));

        head.prev.replace(Some(Rc::clone(&item)));

        self.size += 1;
    }

    pub fn pop_back(&mut self) {
        let prev_value: Option<LinkedListEntry<T>>;

        'tail: {
            let tail = self.tail.borrow();

            let Some(val) = tail.as_ref() else {
                return;
            };

            let prev = val.prev.borrow();

            self.size -= 1;

            let Some(prev) = prev.as_ref() else {
                self.head.replace(None);
                prev_value = None;
                break 'tail;
            };

            prev.next.replace(None);

            prev_value = Some(Rc::clone(prev));
        }

        self.tail.replace(prev_value);
    }

    pub fn pop_front(&mut self) {
        let next_value: Option<LinkedListEntry<T>>;
        'head: {
            let head = self.head.borrow();

            let Some(val) = head.as_ref() else {
                return;
            };

            let next = val.next.borrow();

            self.size -= 1;

            let Some(next) = next.as_ref() else {
                next_value = None;
                self.tail.replace(None);
                break 'head;
            };

            next.prev.replace(None);

            next_value = Some(Rc::clone(next));
        }

        self.head.replace(next_value);
    }

    pub fn first(&self) -> Option<LinkedListEntry<T>> {
        let head = self.head.borrow();

        head.as_ref().map(Rc::clone)
    }

    pub fn last(&self) -> Option<LinkedListEntry<T>> {
        let tail = self.tail.borrow();

        tail.as_ref().map(Rc::clone)
    }

    pub fn get(&self, i: usize) -> Option<LinkedListEntry<T>> {
        let mut curr = self.first()?;

        let mut idx = 0;
        while idx < i {
            let curr_next = curr.next.borrow().as_ref().map(Rc::clone)?;

            curr = curr_next;
            idx += 1;
        }

        Some(curr)
    }

    pub fn iter(&self) -> LinkedListIterator<T> {
        LinkedListIterator {
            first: true,
            curr: self.first(),
        }
    }
}

impl<T> Iterator for LinkedListIterator<T> {
    type Item = LinkedListEntry<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            return self.curr.as_ref().map(Rc::clone);
        }

        let curr = self.curr.as_ref()?;

        let next = curr.next.borrow().as_ref().map(Rc::clone);

        self.curr = next;

        self.curr.as_ref().map(Rc::clone)
    }
}
