#![type_length_limit = "320000000"]

trait Fun {
    fn eval(&self, t: usize) -> usize;
}

fn switch<F: Fun>(f: F) -> impl Fun {
    WrapFn(move |t| f.eval(t))
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
    let anim = switch(anim);

    // 2
    let anim = switch(anim);

    // 4
    let anim = switch(anim);
    let anim = switch(anim);

    // 8
    //    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
    let anim = switch(anim);
    let anim = switch(anim);
    let anim = switch(anim);
    let anim = switch(anim);

    // 16
    //    Finished dev [unoptimized + debuginfo] target(s) in 0.26s
    let anim = switch(anim);
    let anim = switch(anim);
    let anim = switch(anim);
    let anim = switch(anim);
    let anim = switch(anim);
    let anim = switch(anim);
    let anim = switch(anim);
    let anim = switch(anim);

    // 24
    //    Finished dev [unoptimized + debuginfo] target(s) in 55.79s
    let anim = switch(anim);
    let anim = switch(anim);
    let anim = switch(anim);
    let anim = switch(anim);
    let anim = switch(anim);
    let anim = switch(anim);
    let anim = switch(anim);
    let anim = switch(anim);

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
