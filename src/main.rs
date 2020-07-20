#![type_length_limit="3200000000000000"]

use std::marker::PhantomData;

trait Fun {
    fn eval(&self, t: usize) -> usize;
}

fn switch<F: Fun, G: Fun>(f: F, time: usize, g: G) -> impl Fun
where
    G: Fun,
{
    WrapFn(move |t| if t < time { f.eval(t) } else { g.eval(t) })
}

struct WrapFn<F>(F);

impl<F> Fun for WrapFn<F>
where
    F: Fn(usize) -> usize,
{
    fn eval(&self, t: usize) -> usize {
        self.0(t)
    }
}

fn main() {
    println!("Hello, world!");

    let anim = WrapFn(|_| 42);

    // 1
    let anim = switch(anim, 1, WrapFn(|_| 42));

    // 2
    let anim = switch(anim, 1, WrapFn(|_| 42));

    // 4
    let anim = switch(anim, 1, WrapFn(|_| 42));
    let anim = switch(anim, 1, WrapFn(|_| 42));

    // 8
    let anim = switch(anim, 1, WrapFn(|_| 42));
    let anim = switch(anim, 1, WrapFn(|_| 42));
    let anim = switch(anim, 1, WrapFn(|_| 42));
    let anim = switch(anim, 1, WrapFn(|_| 42));

    // 16
    let anim = switch(anim, 1, WrapFn(|_| 42));
    let anim = switch(anim, 1, WrapFn(|_| 42));
    let anim = switch(anim, 1, WrapFn(|_| 42));
    let anim = switch(anim, 1, WrapFn(|_| 42));
    let anim = switch(anim, 1, WrapFn(|_| 42));
    let anim = switch(anim, 1, WrapFn(|_| 42));

    // 32
    let anim = switch(anim, 1, WrapFn(|_| 42));
    let anim = switch(anim, 1, WrapFn(|_| 42));
    let anim = switch(anim, 1, WrapFn(|_| 42));
    let anim = switch(anim, 1, WrapFn(|_| 42));
    let anim = switch(anim, 1, WrapFn(|_| 42));
    let anim = switch(anim, 1, WrapFn(|_| 42));
    let anim = switch(anim, 1, WrapFn(|_| 42));
    let anim = switch(anim, 1, WrapFn(|_| 42));
    let anim = switch(anim, 1, WrapFn(|_| 42));
    let anim = switch(anim, 1, WrapFn(|_| 42));
    let anim = switch(anim, 1, WrapFn(|_| 42));
    let anim = switch(anim, 1, WrapFn(|_| 42));

    println!("{}", anim.eval(0));
}
