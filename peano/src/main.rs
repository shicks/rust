#![recursion_limit="1024"]

use std::marker::PhantomData;

struct Succ<T> {
    phantom: PhantomData<T>,
}

trait Peano {
    //type Pred: PeanoNum;
    fn value() -> u32;
}

impl Peano for () {
    //type Pred = ();
    fn value() -> u32 { 0 }
}

impl<T: Peano> Peano for Succ<T> {
    //type Pred = T;
    fn value() -> u32 { T::value() + 1 }
}





trait AddT<S> {
    type Sum;
}
impl<S> AddT<S> for () {
    type Sum = S;
}
impl<S, T> AddT<S> for Succ<T> where T : AddT<Succ<S>> {
    type Sum = <T as AddT<Succ<S>>>::Sum;
}

trait MulT<S> {
    type Product;
}
impl<S> MulT<S> for () {
    type Product = ();
}
impl<S, T> MulT<S> for Succ<T> where T : MulT<S>, S : AddT<<T as MulT<S>>::Product> {
    type Product = <S as AddT<<T as MulT<S>>::Product>>::Sum;
}


trait GtP<S> {}
impl<T> GtP<()> for Succ<T> {}
impl<S, T> GtP<Succ<S>> for Succ<T> where T : GtP<S> {}

trait GeP<S> {}
impl<T> GeP<()> for T {}
impl<S, T> GeP<Succ<S>> for Succ<T> where T : GeP<S> {}

trait EqP<S> {}
impl EqP<()> for () {}
impl<S, T> EqP<Succ<S>> for Succ<T> where T : EqP<S> {}


trait GeC<S, T, F> {
    type Result;
}
impl<N, T, F> GeC<(), T, F> for N {
    type Result = T;
}
impl<S, T, F> GeC<Succ<S>, T, F> for () {
    type Result = F;
}
impl<S, N, T, F> GeC<Succ<S>, T, F> for Succ<N> where N : GeC<S, T, F> {
    type Result = <N as GeC<S, T, F>>::Result;
}


trait Pred {
    type Result;
}
impl Pred for () {
    type Result = ();
}
impl<T> Pred for Succ<T> {
    type Result = T;
}

trait SubT<S> {
    type Difference;
}
impl<T> SubT<()> for T {
    type Difference = T;
}
impl<S, T> SubT<Succ<S>> for Succ<T> where T : SubT<S> {
    type Difference = <T as SubT<S>>::Difference;
}
impl<S> SubT<Succ<S>> for () {
    type Difference = ();
}

trait MinMax {
    type Min;
    type Max;
}
impl<A, B> MinMax for (A, B) where (A, B) : MinMax_<A, B> {
    type Min = <(A, B) as MinMax_<A, B>>::Min;
    type Max = <(A, B) as MinMax_<A, B>>::Max;
}

trait MinMax_<A, B> {
    type Min;
    type Max;
}
impl<A, B> MinMax_<A, B> for ((), ()) {
    type Min = A;
    type Max = B;
}
impl<A, B, T> MinMax_<A, B> for ((), Succ<T>) {
    type Min = A;
    type Max = B;
}
impl<A, B, T> MinMax_<A, B> for (Succ<T>, ()) {
    type Min = B;
    type Max = A;
}
impl<A, B, S, T> MinMax_<A, B> for (Succ<S>, Succ<T>)
    where (S, T) : MinMax_<A, B> {
    type Min = <(S, T) as MinMax_<A, B>>::Min;
    type Max = <(S, T) as MinMax_<A, B>>::Max;
}

trait DivT<T> {
    type Quotient;
}
impl<S, T> DivT<T> for S
    where Succ<S> : DivT2_<T, ()>,
          <Succ<S> as DivT2_<T, ()>>::Result : Pred {
    type Quotient = <<Succ<S> as DivT2_<T, ()>>::Result as Pred>::Result;
}

trait DivT2_<T, U> {
    type Result;
}
impl<T, U> DivT2_<T, U> for () {
    type Result = U;
}
impl<S, T, U> DivT2_<T, U> for Succ<S>
    where Succ<S> : SubT<T>,
          <Succ<S> as SubT<T>>::Difference : DivT2_<T, Succ<U>> {
    type Result = <<Succ<S> as SubT<T>>::Difference
                   as DivT2_<T, Succ<U>>>::Result;
}


trait ModT<T> {
    type Remainder;
}
impl<S, T> ModT<T> for S
    where S : ModT2_<T, T> {
    type Remainder = <S as ModT2_<T, T>>::Result;
}

trait ModT2_<T, S> {
    type Result;
}
impl<S, T> ModT2_<T, S> for () where T : SubT<S> {
    type Result = <T as SubT<S>>::Difference;
}
impl<T, U> ModT2_<T, Succ<()>> for Succ<U>
    where U : ModT2_<T, T> {
    type Result = <U as ModT2_<T, T>>::Result;
}
impl<S, T, U> ModT2_<T, Succ<Succ<S>>> for Succ<U>
    where U : ModT2_<T, Succ<S>> {
    type Result = <U as ModT2_<T, Succ<S>>>::Result;
}


trait GcdT<A> {
    type Gcd;
}
impl<A, B> GcdT<A> for B
    where (A, B) : MinMax,
          <(A, B) as MinMax>::Min : GcdT2_<<(A, B) as MinMax>::Max> {
    type Gcd = <<(A, B) as MinMax>::Min as GcdT2_<<(A, B) as MinMax>::Max>>::Result;
}

trait GcdT2_<A> {
    type Result;
}
impl<A> GcdT2_<A> for () {
    type Result = A;
}
impl<A, B> GcdT2_<A> for Succ<B>
    where A : ModT<Succ<B>>,
          <A as ModT<Succ<B>>>::Remainder : GcdT2_<Succ<B>> {
    type Result = <<A as ModT<Succ<B>>>::Remainder as GcdT2_<Succ<B>>>::Result;
}

// struct Gcd<A, B>;
// impl<T> Operation for Gcd<(), T> {
//     type Result = T;
// }
// impl<S, T> Operation for Gcd<Succ<S>, T> where

//  HOW TO SHORT-CIRCUIT?!?!?



// trait GcdT<A, B> {
//     type Gcd;
// }
// impl<T> GcdT<(), ()> for T {
//     type Gcd = T;
// }
// impl<S, T>


// trait GcdT<S> {
//     type Gcd;
// }
// impl<T> GcdT<()> for T {
//     type Gcd = T;
// }
// impl<S, T> GcdT<Succ<S>> for T
//     where S : GeC<T, <Succ<S> as GcdT<T>>::Gcd, <<Succ<T> as SubT<S>>::Difference as GcdT<S>>::Gcd>,
//           Succ<S> : GcdT<T>,
//           <Succ<T> as SubT<S>>::Difference : GcdT<S>,
//           Succ<T> : SubT<S> {
//     type Gcd = <S as GeC<T, <Succ<S> as GcdT<T>>::Gcd, <<Succ<T> as SubT<S>>::Difference as GcdT<S>>::Gcd>>::Result;
// }


// PROBLEM - conflicting implementations
//   - we want to define identical impls with exclusive constraints...


//     where S : GeP<T>, Succ<S> : GcdT<T> {
//     type Gcd = <Succ<S> as GcdT<T>>::Gcd;
// }
// impl<S, T> GcdT<S> for Succ<T>
//     where Succ<T> : SubT<S>,
//           <Succ<T> as SubT<S>>::Difference : GcdT<S> {
//     type Gcd = <<Succ<T> as SubT<S>>::Difference as GcdT<S>>::Gcd;
// }


trait Operation {
    type Result;
}


// struct Pred<T> {
//     phantom: PhantomData<T>,
// }
// impl Operation for Pred<()> {
//     type Result = ();
// }
// impl<T> Operation for Pred<Succ<T>> {
//     type Result = T;
// }


struct Add<S, T> {
    phantom: PhantomData<(S, T)>,
}
impl<T> Operation for Add<(), T> {
    type Result = T;
}
impl<S, T> Operation for Add<Succ<S>, T>
    where Add<S, T> : Operation {
    type Result = Succ<<Add<S, T> as Operation>::Result>;
}


// struct Sub<S, T> {
//     phantom: PhantomData<(S, T)>,
// }
// impl<T> Operation for Sub<T, ()> {
//     type Result = T;
// }
// impl<S, T> Operation for Sub<Succ<S>, Succ<T>>
//     //where Sub<S, T> : Operation
// {
//     type Result = Sub<S, T>::Result;
// }


// struct Mul<S, T> {
//     phantom: PhantomData<(S, T)>,
// }
// impl<S, T> Operation for Mul<S, T>
//     where MulHelper<S, T, U> : True {
//     type Result = U;
// }


// impl<T> Operation for Mul<(), T> {
//     type Result = ();
// }
// impl<S, T, U> Operation for Mul<Succ<S>, T>
//     where Mul<S, T> : Operation,
//           Add<T, <Mul<S, T> as Operation>::Result> : Operation {
//     type Result = <Add<T, <Mul<S, T> as Operation>::Result> as Operation>::Result;
// }

trait True {}

struct GreaterEqual<S, T> {
    phantom: PhantomData<(S, T)>,
}
impl<T> True for GreaterEqual<T, ()> {}
impl<S, T> True for GreaterEqual<Succ<S>, Succ<T>>
    where GreaterEqual<S, T> : True {}


struct Adds<S, T, U> {
    phantom: PhantomData<(S, T, U)>,
}
impl<T> True for Adds<(), T, T> {}
impl<S, T, U> True for Adds<Succ<S>, T, Succ<U>> where Adds<S, T, U> : True {}

// struct Muls<S, T, U> {
//     phantom: PhantomData<(S, T, U)>,
// }
// impl<T> True for Muls<(), T, ()> {}
// impl<S, T, U> True for Muls<Succ<S>, T, U>
//     where GreaterEqual<U, T> : True,
//           Adds<, T> : Operation,
//           Muls<S, T, > : True {}


// struct Num<T> {
//     num: u32,
//     phantom: PhantomData<T>,
// }

type Zero = ();
type One = Succ<Zero>;
type Two = Succ<One>;
type Three = Succ<Two>;

type Six = <Add<Three, Three> as Operation>::Result;

type Eight = <Two as AddT<Six>>::Sum;
type Nine = <Three as MulT<Three>>::Product;
type Fourteen = <Six as AddT<Eight>>::Sum;

type EightyOne = <Nine as MulT<Nine>>::Product;

//type Result = <Three as ModT<One>>::Remainder;
type Result = <Fourteen as GcdT<Eight>>::Gcd;

// type ThreeAgain = <Add<(), Three> as Operation>::Result;
// type Six = <Add<Three, Three> as Operation>::Result;

fn main() {
    println!("Hello, Peano {}", <Result as Peano>::value());
}
