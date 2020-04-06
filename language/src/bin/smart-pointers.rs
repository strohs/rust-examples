use std::cell::{Cell, RefCell};
use std::fmt::Debug;
use std::rc::{Rc, Weak};
/// # Smart Pointers Recap
/// * `Rc<T>` enables multiple owners of the same data; `Box<T>` and `RefCell<T>` have single owners.
/// * `Box<T>` allows immutable or mutable borrows checked at compile time; `Rc<T>` allows only
///   immutable borrows checked at compile time; `RefCell<T>` allows immutable or mutable borrows
///   checked at **runtime**
/// * Because `RefCell<T>` allows mutable borrows checked at **runtime**, you can mutate the value
///   inside the `RefCell<T>` even when the `RefCell<T>` is immutable.
///     * RefCell can panic! at runtime if the value it wraps is currently mutably borrowed
///     * However; if you have an `Rc<T>` that holds a `RefCell<T>`, you can get a value that can
///       have multiple owners and that you can mutate!
///
/// # Cells
/// Shareable mutable containers exist to permit mutability in a controlled manner, even in the
/// presence of aliasing. Both `Cell<T>` and `RefCell<T>` allow this in a **single threaded way**.
/// Values of the `Cell<T>` and `RefCell<T>` types may be mutated through shared references
/// (i.e. the common `&T` type)
/// Cell types come in two flavors: `Cell<T>` and `RefCell<T>`. `Cell<T>` implements interior
/// mutability by moving values in and out of the `Cell<T>`. To use references instead of values,
/// one must use the `RefCell<T>` type, acquiring a write lock before mutating. `Cell<T>` provides
/// methods to retrieve and change the current interior value

pub struct Tree<T: Ord + Debug> {
    root: Link<T>,
    len: usize,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;
type WeakLink<T> = Option<Weak<RefCell<Node<T>>>>;

#[derive(Debug)]
enum Color {
    Red,
    Black
}

#[derive(Debug)]
struct Node<T> {
    elem: T,
    color: Color,
    parent: WeakLink<T>,
    left: Link<T>,
    right: Link<T>,
}
impl<T> Node<T> {
    pub fn append(node: &mut Rc<RefCell<Node<T>>>, data: T) -> Option<Rc<RefCell<Node<T>>>> {
        let is_last = node.borrow().right.is_none();
        if is_last {

            let mut new_node = Node { elem:data, color:Color::Black, parent:None, left:None, right:None};
            new_node.parent = Some(Rc::downgrade(&node));
            let rc = Rc::new(RefCell::new(new_node));
            node.borrow_mut().right = Some(rc.clone());
            Some(rc)
        } else {
            // Not the last node, just continue traversing the list:
            if let Some(ref mut next) = node.borrow_mut().right {
                Self::append(next, data)
            } else { None }
        }
    }
}

impl<T> Tree<T> where T: Ord + Debug {
    fn insert(&mut self, elem: T) {
        unimplemented!();
    }
}

fn main() {
    let mut rn = Node { elem:50, color: Color::Black, parent: None, left: None, right: None};
    let rc0 = Rc::new( RefCell::new(rn));
    let mut root = Tree { root: Some(rc0.clone()), len: 0 };

    let mut nn1 = Node { elem:25, color: Color::Red, parent: None, left: None, right: None };
    nn1.parent = Some(Rc::downgrade(&rc0));
    let rc1 = Rc::new( RefCell::new(nn1));
    rc0.borrow_mut().left = Some(rc1.clone());
}