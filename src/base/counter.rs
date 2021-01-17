use core::cell::Cell;

pub trait Counter: Default + Clone {
    unsafe fn incr_strong(&self);
    unsafe fn incr_weak(&self);
    fn decr_strong(&self);
    fn decr_weak(&self);
    fn valid_strong(&self) -> bool;
    fn valid_weak(&self) -> bool;
    fn unique_strong(&self) -> bool;
    fn unique_weak(&self) -> bool;
}

#[derive(Clone, Default)]
pub struct StrongWeakCounter {
    strong: Cell<usize>,
    total: Cell<usize>,
}

impl Counter for StrongWeakCounter {
    unsafe fn incr_strong(&self) {
        let prev = self.strong.get();
        self.strong.set(prev + 1);
        self.incr_weak();
    }
    unsafe fn incr_weak(&self) {
        let prev = self.total.get();
        self.total.set(prev + 1);
    }
    fn decr_strong(&self) {
        let prev = self.strong.get();
        self.strong.set(prev - 1);
        self.decr_weak();
    }
    fn decr_weak(&self) {
        let prev = self.total.get();
        self.total.set(prev - 1);
    }
    fn valid_strong(&self) -> bool {
        self.strong.get() > 0
    }
    fn valid_weak(&self) -> bool {
        self.total.get() > 0
    }
    fn unique_strong(&self) -> bool {
        self.strong.get() <= 1
    }
    fn unique_weak(&self) -> bool {
        self.total.get() <= 1
    }
}
