//! Module for type-level arithmetic.

use std::marker::PhantomData;

/// The Zero type.
pub struct Z;
/// The Succ type.
pub struct S<X> {
    phantom: PhantomData<X>,
}

/// Trait to extract the numeric value from a Peano type.
pub trait Peano {
    /// Returns the numeric value for a type.
    fn value() -> u16;
}

impl Peano for Z {
    fn value() -> u16 { 0 }
}

impl<X> Peano for S<X> where X : Peano {
    fn value() -> u16 { <X as Peano>::value() + 1 }
}

/// Trait for addition: `A + B = <A as AddT<B>>::T`.
pub trait AddT<A> {
    type T;
}
impl<A> AddT<Z> for A {
    type T = A;
}
impl<A, B> AddT<S<B>> for A where A : AddT<B> {
    type T = S<<A as AddT<B>>::T>;
}

/// Trait for subtraction: `A - B = <A as SubT<B>>::T`.
/// Note that negative numbers all reduce to zero.
pub trait SubT<A> {
    type T;
}
impl<A> SubT<Z> for A {
    type T = A;
}
impl<A> SubT<S<A>> for Z {
    type T = Z;
}
impl<A, B> SubT<S<A>> for S<B> where B : SubT<A> {
    type T = <B as SubT<A>>::T;
}

/// Trait for multiplication: `A * B = <A as MulT<B>>::T`.
pub trait MulT<A> {
    type T;
}
impl<A> MulT<A> for Z {
    type T = Z;
}
impl<A, B> MulT<A> for S<B>
    where B : MulT<A>, <B as MulT<A>>::T : AddT<A> {
    type T = <<B as MulT<A>>::T as AddT<A>>::T;
}

/// Trait for integer division: `A div B = <A as DivT<B>>::T`.
pub trait DivT<A> {
    type T;
}
impl<A> DivT<S<Z>> for A {
    type T = A;
}
impl<A, B> DivT<S<S<B>>> for A where A : DivT_<B, Z> {
    type T = <A as DivT_<B, Z>>::T_;
}

/// Internal implementation for division via repeated subtraction.
/// Includes a counter to keep track of the number of times we've
/// subtracted off the divisor.  The divisor is two less, which is
/// fine since dividing by zero is nonsense.
trait DivT_<A, N> {
    type T_;
}
impl<A, N> DivT_<A, N> for Z {
    type T_ = N;
}
impl<A, B, N> DivT_<A, N> for S<B>
    where B : SubT<A>,
          <B as SubT<A>>::T : DivT__<A, N> {
    type T_ = <<B as SubT<A>>::T as DivT__<A, N>>::T_;
}

/// Second internal implementation helper.
trait DivT__<A, N> {
    type T_;
}
impl<A, N> DivT__<A, N> for Z {
    type T_ = N;
}
impl<A, B, N> DivT__<A, N> for S<B> where B : DivT_<A, S<N>> {
    type T_ = <B as DivT_<A, S<N>>>::T_;
}


/// Trait for remainder: `A mod B = <A as ModT<B>>::T`.
pub trait ModT<A> {
    type T;
}
impl<A, B> ModT<B> for A where A : ModT_<B, B> {
    type T = <A as ModT_<B, B>>::T_;
}

/// Internal helper trait for remainder, which keeps
/// track of a looping variable.
trait ModT_<A, N> {
    type T_;
}
impl<A, N> ModT_<A, N> for Z where A : SubT<N> {
    type T_ = <A as SubT<N>>::T;
}
impl<A, B> ModT_<B, S<Z>> for S<A> where A : ModT_<B, B> {
    type T_ = <A as ModT_<B, B>>::T_;
}
impl<A, B, N> ModT_<B, S<S<N>>> for S<A> where A : ModT_<B, S<N>> {
    type T_ = <A as ModT_<B, S<N>>>::T_;
}


/// Trait for greatest common divisor: `gcd(A, B) = <A as GcdT<B>>::T`.
pub trait GcdT<A> {
    type T;
}
impl<A> GcdT<A> for Z {
    type T = A;
}
impl<A, B> GcdT<A> for S<B>
    where A : ModT<S<B>>,
          <A as ModT<S<B>>>::T : GcdT<S<B>> {
    type T = <<A as ModT<S<B>>>::T as GcdT<S<B>>>::T;
}

/// Trait for least common multiple: `lcm(A, B) = <A as LcmT<B>>::T`.
pub trait LcmT<A> {
    type T;
}
impl<A, B> LcmT<A> for B
    where A : MulT<B>,
          A : GcdT<B>,
          <A as MulT<B>>::T : DivT<<A as GcdT<B>>::T> {
    type T = <<A as MulT<B>>::T as DivT<<A as GcdT<B>>::T>>::T;
}
