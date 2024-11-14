use magnus::{function, method, prelude::*, wrap, Ruby};
use magnus::method::RubyFunctionCAry;

#[wrap(class = "Point")]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn x(&self) -> isize {
        self.x
    }

    // fn y(&self) -> isize {
    //     self.y
    // }

    // when set Ruby as first parameter
    // fn y(ruby: &Ruby, rb_self: &Self) -> isize {
    //     let _ = ruby.qtrue();
    //     self.y
    // }
    //     --> src/main.rs:26:9
    //     |
    //  24 |     fn y(ruby: &Ruby, rb_self: &Self) -> isize {
    //     |        - this function doesn't have a `self` parameter
    //  25 |         let _ = ruby.qtrue();
    //  26 |         self.y
    //     |         ^^^^ `self` value is a keyword only available in methods with a `self` parameter
    //     |
    //  help: add a `self` receiver parameter to make the associated `fn` a method
    //     |
    //  24 |     fn y(&self, ruby: &Ruby, rb_self: &Self) -> isize {
    //     |          ++++++
    
    //  warning: unused import: `magnus::method::RubyFunctionCAry`
    //   --> src/main.rs:2:5
    //    |
    //  2 | use magnus::method::RubyFunctionCAry;
    //    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    //    |
    //    = note: `#[warn(unused_imports)]` on by default
    
 

    // when set Ruby as second parameter
    // fn y(&self, ruby: &Ruby) -> isize {
    //     let _ = ruby.qtrue();
    //     self.y
    // }
    //     --> src/main.rs:38:34
    //     |
    //  38 |         class.define_method("y", method!(Point::y, 1))?;
    //     |                                  ^^^^^^^^^^^^^^^^^^^^ method cannot be called on `fn(&Point, &Ruby) -> isize {Point::y}` due to unsatisfied trait bounds
    //     |
    //     = note: the following trait bounds were not satisfied:
    //             `<for<'a, 'b> fn(&'a Point, &'b Ruby) -> isize {Point::y} as FnOnce<(&Ruby, _, _)>>::Output = _`
    //             which is required by `for<'a, 'b> fn(&'a Point, &'b Ruby) -> isize {Point::y}: RubyMethod1<_, _, _>`
    //             `for<'a, 'b> fn(&'a Point, &'b Ruby) -> isize {Point::y}: Fn<(&Ruby, _, _)>`
    //             which is required by `for<'a, 'b> fn(&'a Point, &'b Ruby) -> isize {Point::y}: RubyMethod1<_, _, _>`
    //             `<for<'a, 'b> fn(&'a Point, &'b Ruby) -> isize {Point::y} as FnOnce<(&Ruby, &[Value])>>::Output = _`
    //             which is required by `for<'a, 'b> fn(&'a Point, &'b Ruby) -> isize {Point::y}: RubyFunctionCAry<_>`
    //             `for<'a, 'b> fn(&'a Point, &'b Ruby) -> isize {Point::y}: Fn<(&Ruby, &[Value])>`
    //             which is required by `for<'a, 'b> fn(&'a Point, &'b Ruby) -> isize {Point::y}: RubyFunctionCAry<_>`
    //             `<&for<'a, 'b> fn(&'a Point, &'b Ruby) -> isize {Point::y} as FnOnce<(&Ruby, _, _)>>::Output = _`
    //             which is required by `&for<'a, 'b> fn(&'a Point, &'b Ruby) -> isize {Point::y}: RubyMethod1<_, _, _>`
    //             `&for<'a, 'b> fn(&'a Point, &'b Ruby) -> isize {Point::y}: Fn<(&Ruby, _, _)>`
    //             which is required by `&for<'a, 'b> fn(&'a Point, &'b Ruby) -> isize {Point::y}: RubyMethod1<_, _, _>`
    //             `<&for<'a, 'b> fn(&'a Point, &'b Ruby) -> isize {Point::y} as FnOnce<(&Ruby, &[Value])>>::Output = _`
    //             which is required by `&for<'a, 'b> fn(&'a Point, &'b Ruby) -> isize {Point::y}: RubyFunctionCAry<_>`
    //             `&for<'a, 'b> fn(&'a Point, &'b Ruby) -> isize {Point::y}: Fn<(&Ruby, &[Value])>`
    //             which is required by `&for<'a, 'b> fn(&'a Point, &'b Ruby) -> isize {Point::y}: RubyFunctionCAry<_>`
    //             `&mut for<'a, 'b> fn(&'a Point, &'b Ruby) -> isize {Point::y}: Fn<(_, _)>`
    //             which is required by `&mut for<'a, 'b> fn(&'a Point, &'b Ruby) -> isize {Point::y}: Method1<_, _, _>`
    //             `<&mut for<'a, 'b> fn(&'a Point, &'b Ruby) -> isize {Point::y} as FnOnce<(&Ruby, _, _)>>::Output = _`
    //             which is required by `&mut for<'a, 'b> fn(&'a Point, &'b Ruby) -> isize {Point::y}: RubyMethod1<_, _, _>`
    //             `&mut for<'a, 'b> fn(&'a Point, &'b Ruby) -> isize {Point::y}: Fn<(&Ruby, _, _)>`
    //             which is required by `&mut for<'a, 'b> fn(&'a Point, &'b Ruby) -> isize {Point::y}: RubyMethod1<_, _, _>`
    //             `<&mut for<'a, 'b> fn(&'a Point, &'b Ruby) -> isize {Point::y} as FnOnce<(&Ruby, &[Value])>>::Output = _`
    //             which is required by `&mut for<'a, 'b> fn(&'a Point, &'b Ruby) -> isize {Point::y}: RubyFunctionCAry<_>`
    //             `&mut for<'a, 'b> fn(&'a Point, &'b Ruby) -> isize {Point::y}: Fn<(&Ruby, &[Value])>`
    //             which is required by `&mut for<'a, 'b> fn(&'a Point, &'b Ruby) -> isize {Point::y}: RubyFunctionCAry<_>`
    //     = note: this error originates in the macro `method` (in Nightly builds, run with -Z macro-backtrace for more info)
    
    //     warning: unused import: `magnus::method::RubyFunctionCAry`
    //     --> src/main.rs:2:5
    //     |
    //     2 | use magnus::method::RubyFunctionCAry;
    //     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    //     |
    //     = note: `#[warn(unused_imports)]` on by default
        
    //     For more information about this error, try `rustc --explain E0599`.
    //     warning: `magnus-test` (bin "magnus-test") generated 1 warning
    //     error: could not compile `magnus-test` (bin "magnus-test") due to 1 previous error; 1 warning emitted
 


    fn distance(&self, other: &Point) -> f64 {
        (((other.x - self.x).pow(2) + (other.y - self.y).pow(2)) as f64).sqrt()
    }
}

fn main() -> Result<(), String> {
    Ruby::init(|ruby| {
        let class = ruby.define_class("Point", ruby.class_object())?;
        class.define_singleton_method("new", function!(Point::new, 2))?;
        class.define_method("x", method!(Point::x, 0))?;
        class.define_method("y", method!(Point::y, 0))?;
        class.define_method("distance", method!(Point::distance, 1))?;

        let d: f64 = ruby.eval(
            "a = Point.new(0, 0)
             b = Point.new(5, 10)
             a.distance(b)",
        )?;

        println!("{}", d);
        Ok(())
    })
}