#![type_length_limit = "3200000000000000"]

use std::marker::PhantomData;

trait Fun {
    fn eval(&self, t: usize) -> usize;
}

struct C;

impl Fun for C {
    fn eval(&self, _: usize) -> usize {
        42
    }
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

    let anim = C;

    // 1
    let anim = switch(anim, 1, C);

    // 2
    let anim = switch(anim, 1, C);

    // 4
    let anim = switch(anim, 1, C);
    let anim = switch(anim, 1, C);

    // 8
    let anim = switch(anim, 1, C);
    let anim = switch(anim, 1, C);
    let anim = switch(anim, 1, C);
    let anim = switch(anim, 1, C);

    // 16
    let anim = switch(anim, 1, C);
    let anim = switch(anim, 1, C);
    let anim = switch(anim, 1, C);
    let anim = switch(anim, 1, C);
    let anim = switch(anim, 1, C);
    let anim = switch(anim, 1, C);

    // 32
    let anim = switch(anim, 1, C);
    let anim = switch(anim, 1, C);
    let anim = switch(anim, 1, C);
    let anim = switch(anim, 1, C);
    let anim = switch(anim, 1, C);
    let anim = switch(anim, 1, C);
    let anim = switch(anim, 1, C);
    let anim = switch(anim, 1, C);
    let anim = switch(anim, 1, C);
    let anim = switch(anim, 1, C);
    let anim = switch(anim, 1, C);
    let anim = switch(anim, 1, C);

    println!("{}", anim.eval(0));
}
