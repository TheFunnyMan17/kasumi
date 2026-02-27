// src/macros/steps.rs @ kernel
// no idea where to put this

// pass 1: find out how many steps there are

#[doc(hidden)]
#[macro_export]
macro_rules! __steps_count {
    // base: empty
    () => { 0usize };
    // @skip with a following item
    (@skip $label:literal => $expr:expr , $($rest:tt)*) => {
        $crate::__steps_count!($($rest)*)
    };
    // @skip, last item (trailing comma already consumed or absent)
    (@skip $label:literal => $expr:expr) => { 0usize };
    // active with a following item
    ($label:literal => $expr:expr , $($rest:tt)*) => {
        1usize + $crate::__steps_count!($($rest)*)
    };
    // active, last item
    ($label:literal => $expr:expr) => { 1usize };
}

// pass 2: run the steps

#[doc(hidden)]
#[macro_export]
macro_rules! __steps_run {
    // base: nothing left (with or without trailing comma)
    ($n:ident, $total:ident) => {};
    ($n:ident, $total:ident ,) => {};
    // @skip with a following item
    ($n:ident, $total:ident , @skip $label:literal => $expr:expr , $($rest:tt)*) => {
        $crate::__steps_run!($n, $total , $($rest)*)
    };
    // @skip, last item
    ($n:ident, $total:ident , @skip $label:literal => $expr:expr) => {};
    // active with a following item
    ($n:ident, $total:ident , $label:literal => $expr:expr , $($rest:tt)*) => {
        $expr;
        $n += 1;
        $crate::sprintln!("{}/{} {}", $n, $total, $label);
        $crate::__steps_run!($n, $total , $($rest)*)
    };
    // active, last item
    ($n:ident, $total:ident , $label:literal => $expr:expr) => {
        $expr;
        $n += 1;
        $crate::sprintln!("{}/{} {}", $n, $total, $label);
    };
}

// the public macro for convenience :)

#[macro_export]
macro_rules! steps {
    ($($tt:tt)*) => {{
        let _total: usize = $crate::__steps_count!($($tt)*);
        let mut _n: usize = 0;
        $crate::__steps_run!(_n, _total , $($tt)*);
    }};
}
