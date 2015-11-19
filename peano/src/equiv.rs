//! Equivalence class arithmetic.

// extern crate num;

// use num::traits::{Bounded,One,Zero};
// use num::{NumCast,ToPrimitive};
use peano::{Peano,LcmT};
use std::fmt;
use std::fmt::{Display,Formatter};
use std::marker::PhantomData;
use std::ops::{Add,Mul,Neg,Div,Sub};
use std::char::from_u32;

#[derive(Copy,PartialEq,Eq,PartialOrd,Ord,Clone,Debug)]
#[allow(raw_pointer_derive)]
pub struct Mod<N> {
    pub value: u16,
    phantom: PhantomData<*const N>,
}

impl<N: Peano> Mod<N> {
    pub fn new(value: u16) -> Self {
        Mod{value: value % <N as Peano>::value(), phantom: PhantomData}
    }
    pub fn base(&self) -> u16 {
        <N as Peano>::value()
    }
}

// impl<N: Peano> NumCast for Mod<N> {
//     fn from<T: ToPrimitive>(n: T) -> Option<Self> {
//         n.to_u16.map(|n| Self::new(n))
//     }
// }

impl<N: Peano> Display for Mod<N> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let base = format!("{}", <N as Peano>::value());
        let base = base.bytes().map(|b| from_u32((b as u32) + 0x2050).unwrap());
        let base : String = base.collect();
        write!(f, "{}{}", self.value, base)
    }
}

// impl<N: Peano> Bounded for Mod<N> {
//     fn min_value() -> Self {
//         Self::new(0)
//     }
//     fn max_value() -> Self {
//         Self::new(Self::value() - 1)
//     }
// }

// impl<N: Peano> One for Mod<N> {
//     fn one() -> Self {
//         Self::new(1)
//     }
// }

// impl<N: Peano> Zero for Mod<N> {
//     fn zero() -> Self {
//         Self::new(0)
//     }
// }

impl<N: Peano> Add for Mod<N> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.value + other.value)
    }
}

impl<N: Peano> Sub for Mod<N> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.value - other.value)
    }
}

impl<N: Peano> Mul for Mod<N> {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self::new(self.value * other.value)
    }
}

impl<N: Peano> Neg for Mod<N> {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(<N as Peano>::value() - self.value)
    }
}

impl<N: Peano> Div for Mod<N> {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self::new(
            self.value * mod_inverse(other.value, <N as Peano>::value()))
    }
}

fn euclid<N>(a: i16, b: i16, u: N, v: N) -> N where 
    N : Sub<Output=N>, N : Mul<i16, Output=N>, N: Copy {
    if b == 0 {
        u
    } else if a < b {
        euclid(b, a, v, u)
    } else {
        euclid(b, a % b, v, u - v * (a / b))
    }
}

fn mod_inverse(value: u16, base: u16) -> u16 {
    let value = value as i16;
    let base = base as i16;
    let mut v = euclid(base, value, 0, 1) % base;
    if v < 0 { v += base }
    v as u16
}


/// Implementation of the Chinese Remainder Theorem
/// TODO(sdh): add requirement that A coprime B?
pub fn crt2<A, B>(a: &Mod<A>, b: &Mod<B>) -> Mod<<A as LcmT<B>>::T>
    where A : Peano, B : Peano, A : LcmT<B>, <A as LcmT<B>>::T : Peano {
    let mod_a = <A as Peano>::value();
    let mod_b = <B as Peano>::value();
    let crt_a = mod_b;
    let crt_b = mod_a;
    let inv_a = mod_inverse(crt_a, mod_a);
    let inv_b = mod_inverse(crt_b, mod_b);
    let res = crt_a * a.value * inv_a + crt_b * b.value * inv_b;
    Mod::new(res)
}

pub fn crt3<A, B, C>(a: &Mod<A>, b: &Mod<B>, c: &Mod<C>)
                     -> Mod<<<A as LcmT<B>>::T as LcmT<C>>::T>
    where A : Peano, B : Peano, C : Peano,
          A : LcmT<B>, <A as LcmT<B>>::T : LcmT<C>,
          <<A as LcmT<B>>::T as LcmT<C>>::T : Peano {
    let mod_a = <A as Peano>::value();
    let mod_b = <B as Peano>::value();
    let mod_c = <C as Peano>::value();
    let crt_a = mod_b * mod_c;
    let crt_b = mod_a * mod_c;
    let crt_c = mod_a * mod_b;
    let inv_a = mod_inverse(crt_a, mod_a);
    let inv_b = mod_inverse(crt_b, mod_b);
    let inv_c = mod_inverse(crt_c, mod_c);
    let res =
        crt_a * a.value * inv_a
            + crt_b * b.value * inv_b
            + crt_c * c.value * inv_c;
    Mod::new(res)
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_crt2() {
        let a: Mod<S<S<S<Z>>>> = Mod::new(1);
        let b: Mod<S<S<S<S<S<Z>>>>>> = Mod::new(3);
        let n = crt2(&a, &b);
        assert_eq!(15, n.base());
        assert_eq!(13, n.value);
    }

}
