#[macro_export]
macro_rules! from_chain {
    ($to:ty : $via:ident, $from:ty $(: $converter:ident, $next:ty)+) => (
        from_many!($to : $via, $from, $($next),+);
        from_chain!($from $(: $converter, $next)+);
    );
    ($to:ty : $via:ident, $from:ty) => (
        from!($to = $via($from));
    );
}

#[macro_export]
macro_rules! from_many {
    ($to:ty : $via:ident, $from:ty, $( $continue:ty ),+ $(,)?) => (
        from!($to = $via($from));
        from_many!($to : $via, $( $continue ),+);
    );
    ($to:ty : $via:ident, $from:ty $(,)?) => (
        from!($to = $via($from));
    );
}
