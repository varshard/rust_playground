use std::cell::RefCell;
use std::rc::Rc;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}


/* helper type,
    Option, nullable
    Rc, reference count,
    RefCell, allow mutate through shared reference, allow dynamically borrowing (temporarily claim exclusive mutable access)
*/
type Link<T> = Option<Rc<RefCell<Node<T>>>>;

pub struct Node<T> {
    value: T,
    // use box, because it's not possible to predetermine the size of next, so use heap
    next: Link<T>,
    prev: Link<T>
}

impl<T> Node<T> {
    fn new(val: T) -> Self {
        Self{ value: val, next: None, prev: None}
    }
}

pub struct LinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    size: usize,
}

impl<T> LinkedList<T> {
    fn length(&self) -> usize {
        self.size
    }

    fn is_empty(&self) -> bool {
        self.length() == 0
    }

    fn new() -> Self {
        Self{
            head: None,
            tail: None,
            size: 0,
        }
    }

    fn append(&mut self, val: T) {
        let node = Rc::new(RefCell::new(Node::new(val)));
        /*
        this is equivalent to
        let prev_tail = self.tail.take();
        match prev_tail {
            Some(t),
            None => {}
        }
        */
        if let Some(prev_tail) = self.tail.take() {
            // can't make next = Some(node), directly, because it would move node and make self.tail assignment not possible
            prev_tail.borrow_mut().next = Some(Rc::clone(&node));
            node.borrow_mut().prev = Some(prev_tail);
            self.tail = Some(node);
            self.size += 1;
        } else {
            self.head = Some(Rc::clone(&node));
            self.tail = Some(node);
            self.size = 1;
        }

    }

    fn pop_front(&mut self) -> Option<T> {
        /* take will take a mutable reference. It steal the value and put None in its place. map() will move the value out of self.head, which isn't allowed.
        So, we need to steal it from head.
        * map is an Option's function, it will apply the function to Some or return None if the head is none
        So, using map here will
        1. if head is None, then return
        2. if not, then apply the function to the item
        */
        let prev_head = self.head.take();
        prev_head.map(|prev_head| {
            self.size -= 1;

            /* get the next node,
            Need take(), because the next_head is borrowed by Some(next_head).
            take will put None in prev_head.next to take ownership of the next_head`
             */
            match prev_head.borrow_mut().next.take() {
                Some(next_head) => {
                    next_head.borrow_mut().prev = None;
                    self.head = Some(next_head);
                }
                None => {
                    // make tail = None
                    self.tail = None;
                }
            }

            // unwrap and return the previous head's value
            Rc::try_unwrap(prev_head).ok().unwrap().into_inner().value
        })

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn append_single_item() {
        let mut list = LinkedList::new();
        list.append(2);
        assert_eq!(list.size, 1);
        assert_eq!(list.pop_front().unwrap(), 2);
    }

    #[test]
    fn append_2_items() {
        let mut list = LinkedList::new();
        list.append(2);
        list.append(3);
        assert_eq!(list.size, 2);
        assert_eq!(list.pop_front().unwrap(), 2);
        assert_eq!(list.size, 1);
    }
}
