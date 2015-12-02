use std::marker::PhantomData;

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

trait Is<A> {}
impl<A> Is<A> for A {}

unsafe fn runtime_transmute<T, U>(t: T) -> U {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<U>());
    std::ptr::read(&t as *const _ as *const _)
}

trait FromStr {
    fn from_str(String) -> Self;
}

struct Monad<Type, Elem> {
    phantom: PhantomData<(Type, Elem)>
}

// trait Monad {
//     type Type;
//     type A;
//     type M;
//     fn ret(a: Self::A) -> Self::M;
//     fn fail(msg: &str) -> Self::M;
//     fn bind<B, MB, F>(ma: Self::M, f: F) -> MB
//         where MonadWrapper<Self::Type, B> : Monad,
//               MB : Is<<MonadWrapper<Self::Type, B> as Monad>::M>,
//               F : Fn(Self::A) -> MB;
// }

struct Maybe;

impl<A> Monad<Maybe, A> {
    fn ret(a: A) -> Option<A> { Some(a) }
    fn bind<B, F: Fn(A) -> Option<B>>(ma: Option<A>, f: F) -> Option<B> {
        match ma {
            Some(a) => f(a),
            None => None::<B>,
            //None => (None as Option<B>) as <B as MonadType<Maybe>>::M,
            //None => unsafe { mem::transmute(None as Option<B>) },
        }
    }
    fn fail(_: &str) -> Option<A> { None }
}

struct List;
impl<A> Monad<List, A> {
    fn ret(a: A) -> Vec<A> { vec![a] }
    fn bind<B, F: Fn(A) -> Vec<B>>(ma: Vec<A>, f: F) -> Vec<B> {
        let mut out = vec![];
        for a in ma {
            for b in f(a) {
                out.push(b);
            }
        }
        out
    }
    fn fail(_: &str) -> Vec<A> { vec![] }
}

// trait Monad<Type> {
//     type M;
//     fn ret(self) -> Self::M;
//     fn fail(msg: &str) -> Self::M;
//     fn bind<B, F>(_unused: Option<Self>, ma: Self::M, f: F) -> <B as Monad<Type>>::M
//         where B : Monad<Type>, F : Fn(Self) -> <B as Monad<Type>>::M;
// }

// struct Maybe;

// impl<A> Monad<Maybe> for A {
//     type M = Option<A>;
//     fn ret(self) -> Option<A> { Some(self) }
//     fn bind<B, F>(_unused: Option<A>, ma: Option<A>, f: F) -> <B as Monad<Maybe>>::M
//         where B : Monad<Maybe>, F : Fn(A) -> <B as Monad<Maybe>>::M {
//         match ma {
//             Some(a) => f(a),
//             None => runtime_transmute(None::<B>),
//             //None => (None as Option<B>) as <B as MonadType<Maybe>>::M,
//             //None => unsafe { mem::transmute(None as Option<B>) },
//         }
//     }
//     fn fail(_: &str) -> Option<A> { None }
// }

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
    (   < $mt: ty >
        $p: pat =<< $e: expr; $($t: tt)*
    ) => (
        Monad::<$mt, _>::bind($e, |$p| mdo! { < $mt > $($t)* })
    );

    (   < $mt: ty >
        $p: ident: $ty: ty =<< $e: expr; $($t: tt)*
    ) => (
        Monad::<$mt, _>::bind($e, |$p: $ty| mdo! { < $mt > $($t)* })
    );

    (   < $mt: ty >
        $e: expr; $(t: tt)*
    ) => (
        Monad::<$mt, _>::bind($e, |_| mdo! { < $mt > $($t)* })
    );

    ( < $mt: ty > $e: expr) => ($e);
}

/*
let out = mdo!{
  y: u8 <- half(8);
  half(y)
}

 */

fn main() {

    let my: Option<u8> = mdo! {
        <Maybe>
        y: u8 =<< half(7);
        half(y)
    };

    let opts: Vec<(u8, u8)> = mdo! {
        <List>
        x: u8 =<< vec![1, 2, 3];
        y: u8 =<< vec![4, 5, 6];
        ret((x, y))
    };

    // let x: u8 = 8;
    // let mx: Option<u8> = half(x);
    // let my: Option<u8> = Monad::bind(mx, half);

    println!("Result: {:?}", opts);
}
