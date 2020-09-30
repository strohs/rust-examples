use std::cell::UnsafeCell;

/// Cell is a shareable mutable container. Cells provide 'interior mutability'
///
/// Shareable mutable containers exist to permit mutability in a controlled manner, even in the
/// presence of aliasing. Cell is typically used with types that are `Copy` since `Cell.get()`
/// returns a Copy of type T
/// Cell is NOT thread-safe
pub struct Cell<T> {
    value: UnsafeCell<T>,
}

// implied by UnsafeCell
//impl<T> !Sync for Cell<T> {}

impl<T> Cell<T> {
    pub fn new(value :T) -> Self {
        Cell {
            value: UnsafeCell::new(value)
        }
    }

    pub fn set(&self, value: T) {
        // SAFETY: we know no-one else is concurrently mutating self.value (because !Sync)
        // SAFETY: we know we're not invalidating any references, because we never give any out
        unsafe {
            *self.value.get() = value;
        }
    }

    pub fn get(&self) -> T
        where T: Copy
    {
        // SAFETY: we know no-one else is modifying this value, since only this thread can mutate
        // (because !Sync), and it is executing this function instead.
        unsafe {
            *self.value.get()
        }
    }
}

// #[cfg(test)]
// mod test {
//
//     use super::Cell;
//     use std::sync::Arc;
//     use std::thread;
//
//
//     #[test]
//     /// this test will fail due to threads interleaving their writes to `x`
//     fn bad() {
//         let x = Arc::new(Cell::new(0));
//         let x1 = Arc::clone(&x);
//         let jh1 = thread::spawn(move || {
//             for _ in 0..100_000 {
//                 let x = x1.get();
//                 x1.set(x + 1);
//             }
//         });
//         let x2 = Arc::clone(&x);
//         let jh2 = thread::spawn(|| {
//             for _ in 0..100_000 {
//                 let x = x2.get();
//                 x2.set(x + 1);
//             }
//         });
//         jh1.join().unwrap();
//         jh2.join().unwrap();
//         assert_eq!(x.get(), 2_000_000);
//     }
//
//     #[test]
//     fn bad2() {
//         let x = Cell::new(vec![42]);
//         let first = &x.get()[0];
//         x.set(vec![]);
//         eprintln!("{}", first);
//     }
// }