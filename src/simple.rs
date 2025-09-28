/// Generates simple `From` implemeentations between various types.
///
/// There are currently three variants:
///
/// 1) the struct is wrapped in an enum variant as is. Example:
///
/// ```
/// use treeerror::from;
///
/// struct B;
///
/// enum A {
///     Variant(B),
/// }
/// from!(A = Variant(B));
/// ```
///
/// 2) A shorthand for `From` implementations
///
/// ```
/// use treeerror::from;
///
/// struct C;
///
/// struct B(C);
/// from!(B = |c: C| { B(c) });
/// ```
///
/// 3) shorthand for from where another can then turn into the final
///    type (structures such as `a.into().into()`)
///
/// ```
/// use treeerror::from;
///
/// struct C;
///
/// struct B(C);
/// from!(B = |c: C| {
///     B(c)
/// });
///
/// enum A {
///     Variant(B),
/// }
/// from!(A = Variant(B));
/// from!(A = C > B);
/// ```
#[macro_export]
macro_rules! from {
    ($to:ty = $via:ident ( $from:ty )) => (
        from!($to = |e: $from| {
            Self::$via(e.into())
        });
    );
    ($to:ty = $from:ty > $via:ty) => (
        from!($to = |e: $from| {
            <$via>::from(e).into()
        });
    );
    ($to:ty = |$var:ident: $from:ty| $($tokens:expr)*) => (
        impl From<$from> for $to {
            fn from($var: $from) -> Self {
                $($tokens)*
            }
        }
    );
}

#[cfg(test)]
mod test {
    #[derive(PartialEq, Eq, Debug)]
    enum Root {
        A(A),
    }
    #[derive(PartialEq, Eq, Debug)]
    enum A {
        Alpha(Alpha),
    }
    #[derive(PartialEq, Eq, Debug)]
    enum Alpha {
        One(One),
    }
    #[derive(PartialEq, Eq, Debug)]
    enum One {
        Child(Child),
    }
    #[derive(PartialEq, Eq, Debug)]
    struct Child;

    from!(One = Child(Child));
    from!(Alpha = One(One));
    from!(Alpha = One(Child));
    from!(A = Alpha(Alpha));
    from!(A = Alpha(One));
    from!(A = Alpha(Child));
    from!(Root = A(A));
    from!(Root = A(Alpha));
    from!(Root = A(One));
    from!(Root = A(Child));

    #[test]
    fn test_from_impls() {
        let _: One = Child.into();
        let _: Alpha = One::from(Child).into();
        let _: A = Alpha::from(Child).into();
        let _: Root = Root::from(Child).into();
    }
}
