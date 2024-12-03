use crate::linked_list::LinkedList;
use std::fmt::Display;

mod linked_list;

fn print_linked_list<T: Display>(list: &LinkedList<T>) {
    let mut as_string: &str = &list
        .iter()
        .fold(String::new(), |acc, curr| format!("{}{}, ", acc, curr));

    if list.size != 0 {
        as_string = &as_string[0..as_string.len() - 2];
    }

    println!("LinkedList[{}]", as_string);
}

fn main() {
    let mut ll: LinkedList<i32> = LinkedList::new();
    print_linked_list(&ll);

    println!("First: {:?}", ll.first());
    println!("Last: {:?}", ll.last());

    ll.push_back(2);
    ll.push_back(4);

    ll.push_front(6);
    ll.push_front(8);

    print_linked_list(&ll);

    println!("First: {:?}", ll.first());
    println!("Last: {:?}", ll.last());
    println!("At index 1: {:?}", ll.get(1));

    ll.pop_back();
    ll.pop_front();

    print_linked_list(&ll);

    println!("First: {:?}", ll.first());
    println!("Last: {:?}", ll.last());
    println!("At index 1: {:?}", ll.get(1));
}
