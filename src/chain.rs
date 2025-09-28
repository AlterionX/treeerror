/// Takes multiple enums that wrap child enums (that can be constructed like
/// `A::BVariant(B::CVariant(C))`) and implements `From` for all types
/// involved so that the lowest type can be converted into a higher type.
///
/// The interface declares a series of mappings, from one class to the next. For
/// example, this:
/// ```
/// use treeerror::from;
///
/// struct D;
///
/// enum C {
///     DVariant(D),
/// }
/// from!(C = DVariant(D));
///
///
/// enum B {
///     CVariant(C),
/// }
/// from!(B = CVariant(C));
/// from!(B = D > C);
///
/// enum A {
///     BVariant(B),
/// }
/// from!(A = BVariant(B));
/// from!(A = C > B);
/// from!(A = D > C);
///
/// let c: C = D.into();
/// let b: B = D.into();
/// let b: B = C::from(D).into();
/// let a: A = D.into();
/// let a: A = C::from(D).into();
/// let a: A = B::from(D).into();
/// ```
///
/// is equivalent to:
/// ```
/// use treeerror::from_chain;
///
/// struct D;
///
/// enum C {
///     DVariant(D),
/// }
///
/// enum B {
///     CVariant(C),
/// }
///
/// enum A {
///     BVariant(B),
/// }
///
/// from_chain!(A : BVariant, B : CVariant, C : DVariant, D);
///
/// let c: C = D.into();
/// let b: B = D.into();
/// let b: B = C::from(D).into();
/// let a: A = D.into();
/// let a: A = C::from(D).into();
/// let a: A = B::from(D).into();
/// ```
#[macro_export]
macro_rules! from_chain {
    ($to:ty : $via:ident, $from:ty $(: $converter:ident, $next:ty)+) => (
        $crate::from_many!($to : $via, $from, $($next),+);
        $crate::from_chain!($from $(: $converter, $next)+);
    );
    ($to:ty : $via:ident, $from:ty) => (
        $crate::from!($to = $via($from));
    );
}

/// Implements `From` multiple times.
///
/// You likely want to use `from_chain!` instead of `from_many!` since `from_many!`
/// omits several useful `From` implementations and typically does not work alone.
///
/// This, for example, fails to compile:
/// ```compile_fail
/// use treeerror::from_many;
///
/// struct D;
///
/// enum C {
///     DVariant(D),
/// }
///
///
/// enum B {
///     CVariant(C),
/// }
///
/// enum A {
///     BVariant(B),
/// }
///
/// from_many!(A : BVariant, B, C, D);
/// ```
///
/// And you need something like this instead:
/// ```
/// use treeerror::from_many;
///
/// struct D;
///
/// enum C {
///     DVariant(D),
/// }
///
/// // You can also use `from!(C = DVariant(D))`
/// from_many!(C : DVariant, D);
///
/// enum B {
///     CVariant(C),
/// }
///
/// from_many!(B : CVariant, C, D);
///
/// enum A {
///     BVariant(B),
/// }
///
/// from_many!(A : BVariant, B, C, D);
///
/// // commented out tests are ones that work with `from_chain!`
/// let c: C = D.into();
/// // let b: B = D.into();
/// let b: B = C::from(D).into();
/// // let a: A = D.into();
/// // let a: A = C::from(D).into();
/// let a: A = B::from(C::from(D)).into();
/// ```
///
/// There is also a variant that does conversion via an intermediate type.
#[macro_export]
macro_rules! from_many {
    ($to:ty : $via:ident, $from:ty, $($continue:ty),+) => (
        $crate::from!($to = $via($from));
        $crate::from_many!($to : $via, $($continue),+);
    );
    ($to:ty : $via:ident, $from:ty $(,)?) => (
        $crate::from!($to = $via($from));
    );
    ($to:ty = $from:ty, $($continue:ty),+ > $via:ty) => (
        $crate::from!($to = $from > $via);
        $crate::from_many!($to = $($continue),+ > $via);
    );
    ($to:ty = $from:ty $(,)? > $via:ty) => (
        $crate::from!($to = $from > $via);
    );
}
