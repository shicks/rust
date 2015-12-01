// use std::marker::PhantomData;

// TODO(sdh): can we partially specify types?



// fn do_something(arg: Vec<u8>) -> Vec<u16> {

//     mdo!{
//         x: u8 <- some_monad();
//         y: u16 <- other_monad(x);
//         finally(y)
//     }
// }

// bind(some_monad(), |x| {
//     bind(other_monad(x), |y| {
//         finally(y)
//     })
// })

trait FromStr {
    fn from_str(String) -> Self;
}


// struct Witness<T> {
//     phantom: PhantomData<T>,
// }


// trait Monad<Type> {
//     type T;
//     type M;
//     fn ret(self) -> Self::M;
//     fn bind<B, F>(ma: Self::M, f: F) -> <B as Monad<Type>>::M
//         where B: Monad<Type>, F: Fn(Self::T) -> <B as Monad<Type>>::M;
//     fn fail(msg: &str) -> Self::M;
// }

// struct Maybe;

// impl<A> Monad<Maybe> for A {
//     type T = A;
//     type M = Option<A>;
//     fn ret(self) -> Option<A> { Some(self) }
//     fn bind<B: Monad<Maybe>, F: Fn(A) -> <B as Monad<Maybe>>::M>(ma: Option<A>, f: F) -> <B as Monad<Maybe>>::M {
//         match ma {
//             Some(a) => f(a),
//             None => <B as Monad<Maybe>>::fail(""),
//         }
//     }
//     fn fail(_: &str) -> Option<A> { None }
// }

// trait MonadType<T> {
//     type M: Monad<Self>;
// }


trait Monad<Type> {
    type T;
    fn ret(t: Self::T) -> Self;
    fn fail(msg: &str) -> Self;
    // Problem: how to guarantee that MB is something we know about?!?
    //   -- would like to add extra methods to the trait... but can't?
    //   -- some way to intermediate through Type ?!? but it's not
    //      parametrized, so it doesn't seem possible.
    fn bind<MB, F>(self, f: F) -> MB
        where MB : Monad<Type>, F : Fn(Self::T) -> MB;
    // fn unwrap(self) -> (Type, Witness<T>);
    // fn wrap(t: Type, 
}

struct Maybe;

// impl<A> MonadType<A> for Maybe {
//     type M = Option<A>;
// }

impl<A> Monad<Maybe> for Option<A> {
    type T = A;
    fn ret(a: A) -> Option<A> { Some(a) }
    fn bind<MB, F>(self, f: F) -> MB
        where MB : Monad<Maybe>, F : Fn(A) -> MB {
        match self {
            Some(a) => f(a),
            None => MB::fail(""),
        }
    }
    fn fail(_: &str) -> Option<A> { None }
}

//struct List;

// trait MonadList {
//     type T;
//     fn iter(self) -> Iterator<T>;
//     fn single(a: A) -> Self;
//     fn empty() -> Self;
// }

// trait<A> MonadList for Vec<A> {
//     type T = A;
//     fn iter(self) -> Iterator<A> { self.into_iter() }
//     fn single(a: A) -> Vec<A> { vec![a] }
//     fn empty() -> Vec<A> { vec![] }
// }

// trait<MT: MonadList> Monad<List> for MT {
//     type T = <A as MonadList>::T;
//     fn ret(t: Self::T) -> MT { <MT as MonadList>::single(t) }
//     fn bind<MB: Monad<List>, F: Fn(A) -> MB>(self, f: F) -> MB {
//         for a in <self {
//             for b in f(a) {
             
//             }
//         }
//     }
// }



// trait Monad {
//     type MA;
//     type MB;
//     type A;
//     type B;
//     fn ret(a: Self::A) -> Self::MA;
//     fn bind<F: Fn(Self::A) -> Self::MB>(ma: Self::MA, f: F) -> Self::MB;
//     fn fail(msg: String) -> Self::MA;
// }

// impl<T, S> Monad for (Option<T>, Option<S>) {
//     type MA = Option<T>;
//     type MB = Option<S>;
//     type A = T;
//     type B = S;
//     fn ret(a: T) -> Option<T> { Some(a) }
//     fn bind<F: Fn(T) -> Option<S>>(ma: Option<T>, f: F) -> Option<S> {
//         match ma {
//             Some(a) => f(a),
//             None => None,
//         }
//     }
//     fn fail(_: String) -> Option<T> { None }
// }

// impl<E: FromStr, T, S> Monad for (Result<T, E>, Result<S, E>) {
//     type MA = Result<T, E>;
//     type MB = Result<S, E>;
//     type A = T;
//     type B = S;
//     fn ret(a: T) -> Result<T, E> { Ok(a) }
//     fn bind<F: Fn(T) -> Result<S, E>>(ma: Result<T, E>, f: F) -> Result<S, E> {
//         match ma {
//             Ok(a) => f(a),
//             Err(e) => Err(e),
//         }
//     }
//     fn fail(msg: String) -> Result<T, E> { Err(<E>::from_str(msg)) }
// }

// impl<T, S> Monad for (Vec<T>, Vec<S>) {
//     type MA = Vec<T>;
//     type MB = Vec<S>;
//     type A = T;
//     type B = S;
//     fn ret(a: T) -> Vec<T> { vec![a] }
//     fn bind<F: Fn(T) -> Vec<S>>(ma: Vec<T>, f: F) -> Vec<S> {
//         let mut out: Vec<S> = vec![];
//         for a in ma {
//             for b in f(a) {
//                 out.push(b);
//             }
//         }
//         out
//     }
//     fn fail(_: String) -> Vec<T> { vec![] }
// }

fn half(x: u8) -> Option<u8> {
    if x % 2 == 0 {
        Some(x / 2)
    } else {
        None
    }
}


macro_rules! mdo {
    (
        $p: pat =<< $e: expr; $($t: tt)*
    ) => (
        <_ as Monad<_>>::bind($e, |$p| mdo! { $($t)* })
    );

    (
        $p: ident: $ty: ty =<< $e: expr; $($t: tt)*
    ) => (
        <_ as Monad<_>>::bind($e, |$p: $ty| mdo! { $($t)* })
    );

    (
        $e: expr; $(t: tt)*
    ) => (
        <_ as Monad<_>>::bind($e, |_| mdo! { $($t)* })
    );

    ($e: expr) => ($e);
}

/*
let out = mdo!{
  y: u8 <- half(8);
  half(y)
}

 */

fn main() {

    let my: Option<u8> = mdo! {
        y: u8 =<< half(7);
        half(y)
    };
    //let my = <_ as Monad<_>>::bind(half(x), half);
    match my {
        Some(y) => { println!("Quarter: {}", y); }
        None => { println!("Nothing"); }
    }                   

    println!("Hello, world!");
}
