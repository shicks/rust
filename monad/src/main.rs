#[macro_use]
mod monad;

fn main() {

    let my: Option<u8> = mdo! { <monad::Maybe>
        y: u8 =<< Some(7);
        Some(y)
    };

    let opts: Vec<(u8, u8)> = mdo! { <monad::List>
        Some(x) =<< vec![Some(1), Some(2), None];
        y =<< vec![4, 5, 6];
        vec![(x, y)]
    };

    println!("Result: {:?}", opts);
}
