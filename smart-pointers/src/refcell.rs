use std::cell::{UnsafeCell};
use crate::cell::Cell;
use std::ops::Deref;

#[derive(Copy, Clone)]
enum RefState {
    Unshared,
    Shared(usize),
    Exclusive,
}

// implied by UnsafeCell
// impl<T> !Sync for RefCell<T> {}

pub struct RefCell<T> {
    value: UnsafeCell<T>,
    state: Cell<RefState>,
}

impl<T> RefCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            state: Cell::new(RefState::Unshared),
        }
    }

    pub fn borrow(&self) -> Option<Ref<'_, T>> {
        match self.state.get() {
            RefState::Unshared => {
                self.state.set(RefState::Shared(1));
                // SAFETY: RefCell is not currently shared at all, so it is safe to give out a reference
                Some( Ref{ refcell: self } )
            },
            RefState::Shared(share_count) => {
                // SAFETY: only shared references have been given out, so it's safe to give out another
                self.state.set(RefState::Shared(share_count + 1));
                Some( Ref { refcell: self } )
            },
            RefState::Exclusive => {
                // RefCell is currently exclusively borrowed, so don't give out anything
                None
            }
        }
    }

    pub fn borrow_mut(&self) -> Option<RefMut<'_, T>> {
        if let RefState::Unshared = self.state.get() {
            self.state.set(RefState::Exclusive);
            // SAFETY: no other references have been given out since state would be Shared or Exclusive
            Some( RefMut { refcell: self } )
        } else {
            None
        }
    }
}

pub struct Ref<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<T> std::ops::Deref for Ref<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: a Ref is only created if no exclusive references have been given out. Once it
        // has been given out, state is set to Shared, so no exclusive references are given out.
        // So de-referencing into a shared reference is fine
        unsafe { &*self.refcell.value.get() }
    }
}

impl <T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Shared(1) => {
                self.refcell.state.set(RefState::Unshared);
            },
            RefState::Shared(count) => {
                self.refcell.state.set(RefState::Shared(count - 1));
            },
            RefState::Exclusive | RefState::Unshared  => {
                unreachable!();
            },
        }
    }
}

pub struct RefMut<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<T> std::ops::Deref for RefMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: a RefMut is only created if no shared (or other exclusive) references have been
        // given out. Once it has been given out, state is set to Exclusive, so no shared
        // references are given out. So de-referencing into a mutable reference is fine
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> std::ops::DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.refcell.value.get() }
    }
}

impl <T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Shared(_) | RefState::Unshared => {
                unreachable!();
            },
            RefState::Exclusive => {
                self.refcell.state.set(RefState::Unshared);
            },
        }
    }
}

// todo 1:06:30 Rc()