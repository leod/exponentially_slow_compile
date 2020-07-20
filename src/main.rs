#![type_length_limit="11068707"]

use std::marker::PhantomData;

trait Fun {
    fn eval(&self, t: usize) -> usize;
}

struct Anim<F>(pub F);

impl<F> Anim<F>
where
    F: Fun,
{
    fn switch<G>(self, self_end: usize, next: Anim<G>) -> Anim<impl Fun>
    where
        G: Fun,
    {
        cond(fun(move |t| self_end), self, next)
    }
}

fn cond<F, G, H>(cond: Anim<F>, a: Anim<G>, b: Anim<H>) -> Anim<impl Fun>
where
    F: Fun,
    G: Fun,
    H: Fun,
{
    fun(move |t| if cond.0.eval(t) > 0 { a.0.eval(t) } else { b.0.eval(t) })
}

fn fun(f: impl Fn(usize) -> usize) -> Anim<impl Fun> {
    Anim(WrapFn(f))
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

    let anim = fun(|_| 42);

    // 1
    let anim = anim.switch(1, fun(|_| 42));

    // 2
    let anim = anim.switch(1, fun(|_| 42));

    // 4
    let anim = anim.switch(1, fun(|_| 42));
    let anim = anim.switch(1, fun(|_| 42));

    // 8
    let anim = anim.switch(1, fun(|_| 42));
    let anim = anim.switch(1, fun(|_| 42));
    let anim = anim.switch(1, fun(|_| 42));
    let anim = anim.switch(1, fun(|_| 42));

    // 16
    let anim = anim.switch(1, fun(|_| 42));
    let anim = anim.switch(1, fun(|_| 42));
    let anim = anim.switch(1, fun(|_| 42));
    let anim = anim.switch(1, fun(|_| 42));
    let anim = anim.switch(1, fun(|_| 42));
    let anim = anim.switch(1, fun(|_| 42));

    println!("{}", anim.0.eval(0));
}
