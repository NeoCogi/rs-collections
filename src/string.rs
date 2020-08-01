use crate::vec::*;
use ::core::*;
use ::core::cmp::*;
use crate::hash::*;
use core::fmt::{Arguments, Write};

#[repr(C)]
pub struct String {
    data    : Vec<u8>
}

impl String {
    pub fn with_capacity(c: usize) -> Self {
        Self { data: Vec::with_capacity(c) }
    }

    pub fn new() -> Self { Self { data: Vec::new() } }
    pub fn from(s: &str) -> Self {
        let mut st = Self::new();
        for c in s.bytes() {
            st.data.push(c);
        }
        st
    }

    pub fn as_str(&self) -> &str {
        ::core::str::from_utf8(self.data.as_slice()).expect("Error getting string out")
    }

    pub fn push(&mut self, u: u8) {
        self.data.push(u);
    }

    pub fn into_bytes(self) -> Vec<u8> { self.data }
    pub fn as_bytes(&self) -> &[u8] { self.data.as_slice() }
    pub fn as_mut_vec(&mut self) -> &mut Vec<u8> { &mut self.data }

    pub fn push_str(&mut self, s: &str) {
        for c in s.bytes() {
            self.data.push(c);
        }
    }
}

pub trait Append<T> {
    fn append(&mut self, other: T);
}


impl Append<&String> for String {
    fn append(&mut self, s: &String) {
        for c in s.as_str().bytes() {
            self.data.push(c);
        }
    }
}

impl PartialEq<String> for String {
    fn eq(&self, other: &Self) -> bool {
        let ls = self.data.len();
        let lo = other.data.len();
        if ls != lo { return false }
        for i in 0..self.data.len() {
            if self.data[i] != other.data[i] { return false }
        }
        true
    }
}

impl Eq for String {}

impl PartialEq<&str> for String {
    fn eq(&self, other: &&str) -> bool {
        let ob = (*other).as_bytes();
        let ls = self.data.len();
        let lo = ob.len();
        if ls != lo { return false }
        for i in 0..self.data.len() {
            if self.data[i] != ob[i] { return false }
        }
        true
    }
}

impl Clone for String {
    fn clone(&self) -> Self {
        String::from(self.as_str())
    }
}

impl fmt::Write for String {
    #[inline]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.push_str(s);
        Ok(())
    }

    #[inline]
    fn write_char(&mut self, c: char) -> fmt::Result {
        self.push(c as u8);
        Ok(())
    }
}

impl fmt::Display for String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Hash for String {
    fn hash(&self) -> usize {
        self.as_bytes().hash()
    }
}

pub fn format(args: Arguments<'_>) -> String {
    let mut output = String::new();
    output.write_fmt(args).expect("a formatting trait implementation returned an error");
    output
}

#[macro_export]
macro_rules! format {
    ($fmt:expr, $($args:expr),+) => {
        $crate::string::format(format_args!($fmt, $($args),+))
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_conversion() {
        {
            let u : u32 = 12345;
            if String::from("12345") != format!("{}", u) {
                panic!("fail {}", u);
            }
        }

        {
            let i : i32 = -12345;
            if String::from("-12345") != format!("{}", i) {
                panic!("fail {}", i);
            }
        }
    }
}
