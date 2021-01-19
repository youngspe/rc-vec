use super::counters::WeakType;
use super::generic_vec::GenericVec;
use super::RcVec;
use crate::base::BaseRcVec;
use core::fmt;

impl<T> RcVec<T> {
    pub fn downgrade(&self) -> WeakVec<T> {
        WeakVec {
            base: self.base.try_convert().unwrap(),
        }
    }
}

pub struct WeakVec<T> {
    base: BaseRcVec<WeakType, T>,
}

impl<T> WeakVec<T> {
    pub fn upgrade(&self) -> Option<RcVec<T>> {
        Some(GenericVec {
            base: self.base.try_convert()?,
        })
    }
}

impl<T> Clone for WeakVec<T> {
    fn clone(&self) -> Self {
        Self {
            base: self.base.clone(),
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for WeakVec<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("WeakVec(")?;

        match self.upgrade() {
            Some(v) => fmt::Debug::fmt(&v, f)?,
            None => f.write_str("<dropped>")?,
        }

        f.write_str(")")?;
        Ok(())
    }
}
