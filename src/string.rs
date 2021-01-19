use crate::vec::AcycVec;
use alloc::string::String;
use core::cmp::Ordering;
use core::fmt;
use core::iter;
use core::ops::Index;
use core::ops::{Add, AddAssign};
use core::ops::{Deref, DerefMut};
use core::slice::SliceIndex;

#[derive(Clone)]
pub struct RcString {
    base: AcycVec<u8>,
}

impl RcString {
    pub fn new() -> Self {
        Self {
            base: AcycVec::new(),
        }
    }

    pub fn push(&mut self, c: char) {
        match c.len_utf8() {
            1 => self.base.push(c as u8),
            _ => self.push_str(c.encode_utf8(&mut [0; 4])),
        }
    }

    pub fn push_str<S: AsRef<str>>(&mut self, s: S) {
        self.base.extend_from_slice(s.as_ref().as_bytes());
    }
}

impl From<&str> for RcString {
    fn from(src: &str) -> Self {
        Self {
            base: AcycVec::copy_from_slice(src.as_bytes()),
        }
    }
}

impl AsRef<str> for RcString {
    fn as_ref(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(&self.base) }
    }
}

impl AsMut<str> for RcString {
    fn as_mut(&mut self) -> &mut str {
        unsafe { core::str::from_utf8_unchecked_mut(&mut self.base) }
    }
}

impl Deref for RcString {
    type Target = str;
    fn deref(&self) -> &str {
        self.as_ref()
    }
}

impl DerefMut for RcString {
    fn deref_mut(&mut self) -> &mut str {
        self.as_mut()
    }
}

impl<S: AsRef<str>> AddAssign<S> for RcString {
    fn add_assign(&mut self, rhs: S) {
        self.push_str(rhs.as_ref());
    }
}

impl<S: AsRef<str>> Add<S> for RcString {
    type Output = Self;
    fn add(mut self, rhs: S) -> Self {
        self += rhs;
        self
    }
}

impl<I: SliceIndex<str>> Index<I> for RcString {
    type Output = I::Output;
    fn index(&self, i: I) -> &Self::Output {
        str::index(self, i)
    }
}

impl<S: AsRef<str>> PartialEq<S> for RcString {
    fn eq(&self, rhs: &S) -> bool {
        str::eq(self, rhs.as_ref())
    }
}

impl Eq for RcString {}

impl<S: AsRef<str>> PartialOrd<S> for RcString {
    fn partial_cmp(&self, rhs: &S) -> Option<Ordering> {
        str::partial_cmp(self, rhs.as_ref())
    }
}

impl Ord for RcString {
    fn cmp(&self, rhs: &Self) -> Ordering {
        str::cmp(self, rhs)
    }
}

impl fmt::Debug for RcString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <str as fmt::Debug>::fmt(self, f)
    }
}

impl fmt::Display for RcString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <str as fmt::Display>::fmt(self, f)
    }
}

impl fmt::Write for RcString {
    fn write_str(&mut self, rhs: &str) -> fmt::Result {
        *self += rhs;
        Ok(())
    }
}

impl Extend<char> for RcString {
    fn extend<I: IntoIterator<Item = char>>(&mut self, it: I) {
        for c in it {
            self.push(c);
        }
    }
}

impl<'a> Extend<&'a char> for RcString {
    fn extend<I: IntoIterator<Item = &'a char>>(&mut self, it: I) {
        self.extend(it.into_iter().copied())
    }
}

impl<'a> Extend<&'a str> for RcString {
    fn extend<I: IntoIterator<Item = &'a str>>(&mut self, it: I) {
        for s in it {
            self.push_str(s);
        }
    }
}

impl Extend<RcString> for RcString {
    fn extend<I: IntoIterator<Item = RcString>>(&mut self, it: I) {
        for s in it {
            self.push_str(&s);
        }
    }
}

impl<'a> Extend<&'a RcString> for RcString {
    fn extend<I: IntoIterator<Item = &'a RcString>>(&mut self, it: I) {
        for s in it {
            self.push_str(s);
        }
    }
}

impl Extend<String> for RcString {
    fn extend<I: IntoIterator<Item = String>>(&mut self, it: I) {
        for s in it {
            self.push_str(&s);
        }
    }
}

impl<'a> Extend<&'a String> for RcString {
    fn extend<I: IntoIterator<Item = &'a String>>(&mut self, it: I) {
        for s in it {
            self.push_str(s);
        }
    }
}

impl iter::FromIterator<char> for RcString {
    fn from_iter<I: IntoIterator<Item = char>>(it: I) -> Self {
        let mut this = Self::new();
        this.extend(it);
        this
    }
}

impl<'a> iter::FromIterator<&'a char> for RcString {
    fn from_iter<I: IntoIterator<Item = &'a char>>(it: I) -> Self {
        it.into_iter().copied().collect()
    }
}

impl<'a> iter::FromIterator<&'a str> for RcString {
    fn from_iter<I: IntoIterator<Item = &'a str>>(it: I) -> Self {
        let mut this = Self::new();
        this.extend(it);
        this
    }
}

impl iter::FromIterator<RcString> for RcString {
    fn from_iter<I: IntoIterator<Item = RcString>>(it: I) -> Self {
        let mut it = it.into_iter();
        match it.next() {
            Some(mut s) => {
                s.extend(it);
                s
            }
            None => Self::new(),
        }
    }
}

impl<'a> iter::FromIterator<&'a RcString> for RcString {
    fn from_iter<I: IntoIterator<Item = &'a RcString>>(it: I) -> Self {
        let mut it = it.into_iter();
        match it.next() {
            Some(s) => {
                let mut s = s.clone();
                s.extend(it);
                s
            }
            None => Self::new(),
        }
    }
}

impl iter::FromIterator<String> for RcString {
    fn from_iter<I: IntoIterator<Item = String>>(it: I) -> Self {
        let mut this = Self::new();
        this.extend(it);
        this
    }
}

impl<'a> iter::FromIterator<&'a String> for RcString {
    fn from_iter<I: IntoIterator<Item = &'a String>>(it: I) -> Self {
        let mut this = Self::new();
        this.extend(it);
        this
    }
}
