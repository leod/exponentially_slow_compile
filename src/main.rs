#![type_length_limit="11068707"]

use std::marker::PhantomData;

trait Fun {
    type T;
    type V;

    fn eval(&self, t: Self::T) -> Self::V;
}

struct Anim<F>(pub F);

impl<F> Anim<F>
where
    F: Fun,
    F::T: Copy + PartialOrd,
{
    fn switch<G>(self, self_end: F::T, next: Anim<G>) -> Anim<impl Fun<T = F::T, V = F::V>>
    where
        G: Fun<T = F::T, V = F::V>,
    {
        cond(fun(move |t| t < self_end), self, next)
    }
}

fn cond<T, V, F, G, H>(cond: Anim<F>, a: Anim<G>, b: Anim<H>) -> Anim<impl Fun<T = T, V = V>>
where
    T: Copy,
    F: Fun<T = T, V = bool>,
    G: Fun<T = T, V = V>,
    H: Fun<T = T, V = V>,
{
    fun(move |t| if cond.0.eval(t) { a.0.eval(t) } else { b.0.eval(t) })
}

fn fun<T, V>(f: impl Fn(T) -> V) -> Anim<impl Fun<T = T, V = V>> {
    Anim(WrapFn(f, PhantomData))
}

struct WrapFn<T, V, F: Fn(T) -> V>(F, PhantomData<(T, V)>);

impl<T, V, F> Fun for WrapFn<T, V, F>
where
    F: Fn(T) -> V,
{
    type T = T;
    type V = V;

    fn eval(&self, t: T) -> V {
        self.0(t)
    }
}

fn main() {
    println!("Hello, world!");

    let anim = fun(|_| 42.0);

    // 1
    let anim = anim.switch(0.1, fun(|_| 42.0));

    // 2
    let anim = anim.switch(0.1, fun(|_| 42.0));

    // 4
    let anim = anim.switch(0.1, fun(|_| 42.0));
    let anim = anim.switch(0.1, fun(|_| 42.0));

    // 8
    let anim = anim.switch(0.1, fun(|_| 42.0));
    let anim = anim.switch(0.1, fun(|_| 42.0));
    let anim = anim.switch(0.1, fun(|_| 42.0));
    let anim = anim.switch(0.1, fun(|_| 42.0));

    // 16
    let anim = anim.switch(0.1, fun(|_| 42.0));
    let anim = anim.switch(0.1, fun(|_| 42.0));
    let anim = anim.switch(0.1, fun(|_| 42.0));
    let anim = anim.switch(0.1, fun(|_| 42.0));
    let anim = anim.switch(0.1, fun(|_| 42.0));
    let anim = anim.switch(0.1, fun(|_| 42.0));

    println!("{}", anim.0.eval(0.0));
}
