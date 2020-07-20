#![type_length_limit = "3200000"]

trait Fun {
    fn eval(&self, t: usize) -> usize;
}

struct C;

impl Fun for C {
    fn eval(&self, _: usize) -> usize {
        42
    }
}

fn switch<F: Fun, G: Fun>(f: F, g: G) -> impl Fun {
    WrapFn(move |t| if t < 43 { f.eval(t) } else { g.eval(t) })
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
    let anim = switch(anim, C);

    // 2
    let anim = switch(anim, C);

    // 4
    let anim = switch(anim, C);
    let anim = switch(anim, C);

    // 8
    //    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
    let anim = switch(anim, C);
    let anim = switch(anim, C);
    let anim = switch(anim, C);
    let anim = switch(anim, C);

    // 16
    //    Finished dev [unoptimized + debuginfo] target(s) in 0.26s
    let anim = switch(anim, C);
    let anim = switch(anim, C);
    let anim = switch(anim, C);
    let anim = switch(anim, C);
    let anim = switch(anim, C);
    let anim = switch(anim, C);
    let anim = switch(anim, C);
    let anim = switch(anim, C);

    // 24
    //    Finished dev [unoptimized + debuginfo] target(s) in 55.79s
    let anim = switch(anim, C);
    let anim = switch(anim, C);
    let anim = switch(anim, C);
    let anim = switch(anim, C);
    let anim = switch(anim, C);
    let anim = switch(anim, C);
    let anim = switch(anim, C);
    let anim = switch(anim, C);

    // 32
    //    Takes a long time, I got tired of waiting
    /*let anim = switch(anim, C);
    let anim = switch(anim, C);
    let anim = switch(anim, C);
    let anim = switch(anim, C);
    let anim = switch(anim, C);
    let anim = switch(anim, C);
    let anim = switch(anim, C);
    let anim = switch(anim, C);*/

    println!("{}", anim.eval(0));
}
