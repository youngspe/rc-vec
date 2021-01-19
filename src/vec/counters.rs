use crate::base::{Counter, VecType};
use core::cell::Cell;
use core::convert::Infallible;

#[derive(Clone, Default)]
pub struct StrongWeakCounter {
    strong: Cell<usize>,
    total: Cell<usize>,
}

impl Counter for StrongWeakCounter {}

impl StrongWeakCounter {
    pub fn incr_strong(&self) {
        let prev = self.strong.get();
        self.strong.set(prev + 1);
        self.incr_weak();
    }
    pub fn incr_weak(&self) {
        let prev = self.total.get();
        self.total.set(prev + 1);
    }
    pub fn decr_strong(&self) {
        let prev = self.strong.get();
        self.strong.set(prev - 1);
        self.decr_weak();
    }
    pub fn decr_weak(&self) {
        let prev = self.total.get();
        self.total.set(prev - 1);
    }
    pub fn valid_strong(&self) -> bool {
        self.strong.get() > 0
    }
    pub fn valid_weak(&self) -> bool {
        self.total.get() > 0
    }
    pub fn unique_strong(&self) -> bool {
        self.strong.get() <= 1
    }
    pub fn unique_weak(&self) -> bool {
        self.total.get() <= 1
    }
}
#[derive(Clone, Default)]
pub struct AcyclicCounter {
    count: Cell<usize>,
}

impl Counter for AcyclicCounter {}

impl AcyclicCounter {
    pub fn incr(&self) {
        let prev = self.count.get();
        self.count.set(prev + 1);
    }
    pub fn decr(&self) {
        let prev = self.count.get();
        self.count.set(prev - 1);
    }
    pub fn valid(&self) -> bool {
        self.count.get() > 0
    }
    pub fn unique(&self) -> bool {
        self.count.get() <= 1
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct StrongType(Infallible);

unsafe impl VecType for StrongType {
    type Counter = StrongWeakCounter;

    fn incr(counter: &StrongWeakCounter) {
        counter.incr_strong();
    }
    fn decr(counter: &StrongWeakCounter) {
        counter.decr_strong();
    }
    fn can_take(counter: &StrongWeakCounter) -> bool {
        counter.unique_strong()
    }
    fn can_get_ref(counter: &StrongWeakCounter) -> bool {
        counter.valid_strong()
    }
    fn can_get_mut(counter: &StrongWeakCounter) -> bool {
        counter.unique_weak()
    }
    fn should_drop_entire_vector(counter: &StrongWeakCounter) -> bool {
        !counter.valid_weak()
    }
    fn should_drop_contents(counter: &StrongWeakCounter) -> bool {
        !counter.valid_strong()
    }
    fn can_create(counter: &StrongWeakCounter) -> bool {
        counter.valid_strong()
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct WeakType(Infallible);

unsafe impl VecType for WeakType {
    type Counter = StrongWeakCounter;

    fn incr(counter: &StrongWeakCounter) {
        counter.incr_weak();
    }
    fn decr(counter: &StrongWeakCounter) {
        counter.decr_weak();
    }
    fn can_take(_: &StrongWeakCounter) -> bool {
        false
    }
    fn can_get_ref(_: &StrongWeakCounter) -> bool {
        false
    }
    fn can_get_mut(_: &StrongWeakCounter) -> bool {
        false
    }
    fn should_drop_entire_vector(counter: &StrongWeakCounter) -> bool {
        !counter.valid_weak()
    }
    fn should_drop_contents(_: &StrongWeakCounter) -> bool {
        false
    }
    fn can_create(_: &StrongWeakCounter) -> bool {
        true
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct AcycType(Infallible);

unsafe impl VecType for AcycType {
    type Counter = AcyclicCounter;

    fn incr(counter: &AcyclicCounter) {
        counter.incr();
    }
    fn decr(counter: &AcyclicCounter) {
        counter.decr();
    }
    fn can_take(counter: &AcyclicCounter) -> bool {
        counter.unique()
    }
    fn can_get_ref(counter: &AcyclicCounter) -> bool {
        counter.valid()
    }
    fn can_get_mut(counter: &AcyclicCounter) -> bool {
        counter.unique()
    }
    fn should_drop_entire_vector(counter: &AcyclicCounter) -> bool {
        !counter.valid()
    }
    fn should_drop_contents(counter: &AcyclicCounter) -> bool {
        !counter.valid()
    }
    fn can_create(_: &AcyclicCounter) -> bool {
        true
    }
}
