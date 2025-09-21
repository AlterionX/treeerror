#[macro_export]
macro_rules! from {
    ($to:ty = $via:ident ( $from:ty )) => (
        from!($from > $to |e| {
            Self::$via(e.into())
        });
    );
    ($from:ty > $to:ty |$var:ident| {
        $($tokens:expr)*
    }) => (
        impl From<$from> for $to {
            fn from($var: $from) -> Self {
                $($tokens)*
            }
        }
    );
    ($from:ty > $via:ty > $to:ty) => (
        impl From<$from> for $to {
            fn from(e: $from) -> Self {
                <$via>::from(e).into()
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
