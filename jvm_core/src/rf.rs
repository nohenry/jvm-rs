use std::{
    ops::Deref,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, RwLock, RwLockReadGuard, RwLockWriteGuard,
    },
};

pub struct Rf<T: ?Sized>(pub Arc<RwLock<T>>);

impl<T> PartialEq for Rf<T> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl<T> Eq for Rf<T> {}

unsafe impl<T: ?Sized + Sync + Send> Send for Rf<T> {}
unsafe impl<T: ?Sized + Sync + Send> Sync for Rf<T> {}

impl<T: ?Sized> Clone for Rf<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Rf<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Rf").field(&self.borrow()).finish()
    }
}

impl<T> Deref for Rf<T> {
    type Target = RwLock<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Rf<T> {
    pub fn new(t: T) -> Rf<T> {
        Rf(Arc::new(RwLock::new(t)))
    }

    // pub fn upcast<U: 'static>(self) -> Rf<U> {
    //     Rf(self.0 as Arc<RwLock<U>>)
    // }
}

static mut BACKTRACE: AtomicBool = AtomicBool::new(false);

pub fn set_backtrace(enabled: bool) {
    unsafe { BACKTRACE.store(enabled, Ordering::Relaxed) }
}

impl<T: ?Sized> Rf<T> {
    pub fn borrow_mut(&self) -> RwLockWriteGuard<'_, T> {
        self.0.write().unwrap()
    }

    pub fn borrow(&self) -> RwLockReadGuard<'_, T> {
        if unsafe { BACKTRACE.load(Ordering::Relaxed) } {
            println!("d {}", std::backtrace::Backtrace::force_capture());
        }
        self.0.read().unwrap()
        // self.().unwrap()
    }
}

impl<T> From<T> for Rf<T> {
    fn from(t: T) -> Self {
        Rf::new(t)
    }
}