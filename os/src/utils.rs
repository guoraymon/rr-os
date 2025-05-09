use core::{
    cell::{self, RefCell, UnsafeCell},
    fmt,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
};

pub struct SyncRefCell<T> {
    inner: RefCell<T>,
}

unsafe impl<T> Sync for SyncRefCell<T> {}

impl<T> SyncRefCell<T> {
    pub const fn new(val: T) -> Self {
        Self {
            inner: RefCell::new(val),
        }
    }

    pub fn borrow(&self) -> cell::Ref<'_, T> {
        self.inner.borrow()
    }

    pub fn borrow_mut(&self) -> cell::RefMut<'_, T> {
        self.inner.borrow_mut()
    }
}

pub struct Once<T> {
    initialized: AtomicBool,
    value: UnsafeCell<Option<T>>,
}

impl<T> Once<T> {
    pub const fn new() -> Self {
        Once {
            initialized: AtomicBool::new(false),
            value: UnsafeCell::new(None),
        }
    }

    pub fn call_once(&self, init: fn() -> T) -> &T {
        if !self.initialized.load(Ordering::Acquire) {
            unsafe {
                *self.value.get() = Some(init());
            }
            self.initialized.store(true, Ordering::Release);
        }

        unsafe { (*self.value.get()).as_ref().unwrap() }
    }
}

pub struct Lazy<T> {
    init: fn() -> T,
    once: Once<T>,
}

impl<T> Lazy<T> {
    pub const fn new(init: fn() -> T) -> Self {
        Lazy {
            init,
            once: Once::new(),
        }
    }
}

impl<T> Deref for Lazy<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.once.call_once(self.init)
    }
}

impl<T> DerefMut for Lazy<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.once.call_once(self.init);
        unsafe { (*self.once.value.get()).as_mut().unwrap() }
    }
}

unsafe impl<T: Send + Sync> Sync for Lazy<T> {}

impl<T: fmt::Debug> fmt::Debug for Lazy<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.once.call_once(self.init).fmt(f)
    }
}

#[macro_export]
macro_rules! lazy_static {
    ($(pub static ref $name:ident: $ty:ty = $init:block;)*) => {
        $(
            pub static $name: $crate::utils::Lazy<$ty> = $crate::utils::Lazy::new(|| $init);
        )*
    };
}

pub struct LazyLock<T> {
    init: fn() -> T,
    once: Once<T>,
}

impl<T> LazyLock<T> {
    pub const fn new(init: fn() -> T) -> Self {
        LazyLock {
            init,
            once: Once::new(),
        }
    }
}

impl<T> Deref for LazyLock<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.once.call_once(self.init)
    }
}

impl<T> DerefMut for LazyLock<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.once.call_once(self.init);
        unsafe { (*self.once.value.get()).as_mut().unwrap() }
    }
}

unsafe impl<T: Send + Sync> Sync for LazyLock<T> {}
