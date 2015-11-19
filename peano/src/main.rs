#![recursion_limit="1024"]

mod peano;
mod equiv;

use peano::{S,Z};
use equiv::{Mod,crt2,crt3};

type One = S<Z>;
type Two = S<One>;
type Three = S<Two>;
type Four = S<Three>;
type Five = S<Four>;
type Six = S<Five>;
type Seven = S<Six>;
type Eight = S<Seven>;
type Nine = S<Eight>;
type Ten = S<Nine>;
type Fourteen = S<S<S<S<Ten>>>>;
type Fifteen = S<Fourteen>;
type Twenty = S<S<S<S<S<S<S<S<S<S<Ten>>>>>>>>>>;
type TwentyOne = S<Twenty>;
type Thirty = S<S<S<S<S<S<S<S<S<S<Twenty>>>>>>>>>>;
type ThirtyFive = S<S<S<S<S<Thirty>>>>>;
type Forty = S<S<S<S<S<S<S<S<S<S<Thirty>>>>>>>>>>;
type FortyFive = S<S<S<S<S<Forty>>>>>;
type Fifty = S<S<S<S<S<S<S<S<S<S<Forty>>>>>>>>>>;
type Sixty = S<S<S<S<S<S<S<S<S<S<Fifty>>>>>>>>>>;
type Seventy = S<S<S<S<S<S<S<S<S<S<Sixty>>>>>>>>>>;
type Eighty = S<S<S<S<S<S<S<S<S<S<Seventy>>>>>>>>>>;
type EightyOne = S<Eighty>;
type Ninety = S<S<S<S<S<S<S<S<S<S<Eighty>>>>>>>>>>;
type OneHundred = S<S<S<S<S<S<S<S<S<S<Ninety>>>>>>>>>>;
type OneHundredFive = S<S<S<S<S<OneHundred>>>>>;

// type Nine = <Three as MulT<Three>>::T;
// type Fourteen = <Six as AddT<Eight>>::T;

// type EightyOne = <Nine as MulT<Nine>>::T;

//type Result = <EightyOne as GcdT<Fourteen>>::T;
//type Result = <Five as LcmT<Three>>::T;
//type Result = <Fourteen as GcdT<Eight>>::Gcd;

// type ThreeAgain = <Add<(), Three> as Operation>::Result;
// type Six = <Add<Three, Three> as Operation>::Result;

fn main() {
    // let x: Mod<EightyOne> = Mod::new(5);
    // let one: Mod<EightyOne> = Mod::new(1);
    // let inv = one / x;

    // println!("Hello, Peano {}", <Result as Peano>::value());
    // println!("Hello {}", inv);

    // let a: Mod<Fifteen> = Mod::new(11);
    // let b: Mod<TwentyOne> = Mod::new(5);
    // let c: Mod<ThirtyFive> = Mod::new(24);
    // let n: Mod<OneHundredFive> = crt(a, crt(b, c));

    // let a: Mod<Six> = Mod::new(5);
    // let b: Mod<Ten> = Mod::new(7);
    // let c: Mod<Fifteen> = Mod::new(11);
    // let n: Mod<Thirty> = crt(a, crt(b, c));

    let a: Mod<Nine> = Mod::new(5);
    let b: Mod<Five> = Mod::new(3);
    let n: Mod<FortyFive> = crt2(&a, &b);
    println!(
        "Chinese remainder theorem {}: ({}, {}) => {}", n.base(), a, b, n);

    let a: Mod<Nine> = Mod::new(5);
    let b: Mod<Five> = Mod::new(3);
    let c: Mod<Seven> = Mod::new(1);
    let n = crt3(&a, &b, &c);
    println!(
        "Chinese remainder theorem {}: ({}, {}, {}) => {}",
        n.base(), a, b, c, n);

    let a: Mod<Five> = Mod::new(3);
    let b: Mod<Seven> = Mod::new(5);
    let c: Mod<Eight> = Mod::new(4);
    let n = crt3(&a, &b, &c);
    println!(
        "Chinese remainder theorem {}: ({}, {}, {}) => {}",
        n.base(), a, b, c, n);
}
