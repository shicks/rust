use std;

trait Is<A> {}
impl<A> Is<A> for A {}

// pub trait MonadZero {
//     pub munit() -> Self
//     pub mzero() -> Self
// }

unsafe fn runtime_transmute<T, U>(t: T) -> U {
    assert_eq!(std::mem::size_of::<T>(), std::mem::size_of::<U>());
    std::ptr::read(&t as *const _ as *const _)
}

pub struct Monad<Type, Elem> {
    phantom: std::marker::PhantomData<(Type, Elem)>
}

pub struct Maybe;
impl<A> Monad<Maybe, A> {
    pub fn ret(a: A) -> Option<A> { Some(a) }
    pub fn fail(_: &str) -> Option<A> { None }
    pub fn bind<B, F: Fn(A) -> Option<B>>(ma: Option<A>, f: F) -> Option<B> {
        match ma {
            Some(a) => f(a),
            None => None::<B>,
        }
    }
}

// impl<A> MonadZero for Option<A> {
//     pub fn munit() { Some(()) }
//     pub fn mzero() { None }
// }

pub struct List;
impl<A> Monad<List, A> {
    pub fn ret(a: A) -> Vec<A> { vec![a] }
    pub fn fail(_: &str) -> Vec<A> { vec![] }
    pub fn bind<B, F: Fn(A) -> Vec<B>>(ma: Vec<A>, f: F) -> Vec<B> {
        let mut out = vec![];
        for a in ma {
            for b in f(a) {
                out.push(b);
            }
        }
        out
    }
}

// impl<A> MonadZero for Option<A> {
//     pub fn mzero() { vec![] }
// }

// pub fn guard<T: MonadZero>(cond: bool) -> T {
    
// }

macro_rules! mdo {
    (   < $mt: ty >
        $p: pat =<< $e: expr; $($t: tt)*
    ) => (
        monad::Monad::<$mt, _>::bind($e, |x| {
            match x {
                $p if true => mdo! { < $mt > $($t)* },
                _ => monad::Monad::<$mt, _>::fail("Pattern match failed")
            }
        })
    );

    (   < $mt: ty >
        $p: ident: $ty: ty =<< $e: expr; $($t: tt)*
    ) => (
        monad::Monad::<$mt, _>::bind($e, |$p: $ty| mdo! { < $mt > $($t)* })
    );

    (   < $mt: ty >
        $e: expr; $(t: tt)*
    ) => (
        monad::Monad::<$mt, _>::bind($e, |_| mdo! { < $mt > $($t)* })
    );

    ( < $mt: ty > $e: expr) => ($e);
}
