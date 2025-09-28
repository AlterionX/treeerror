/// Shorthand for mapping one error enum on top of another one.
///
/// This takes things like:
/// ```
/// struct Child0;
/// struct Child1;
/// struct Child2;
/// struct Child3;
/// struct Child4;
///
/// enum Root {
///     ChildVariant0(Child0),
///     ChildVariant1(Child1),
///     ChildVariant2(Child2),
///     ChildVariant3(Child3),
///     ChildVariant4(Child4),
/// }
///
/// enum SimilarRoot {
///     ChildVariant0(Child0),
///     ChildVariant1(Child1),
///     ChildVariant2(Child2),
///     ChildVariant3(Child3),
///     ChildVariant4(Child4),
/// }
///
/// impl From<SimilarRoot> for Root {
///     fn from(r: SimilarRoot) -> Self {
///         match r {
///             SimilarRoot::ChildVariant0(c) => Self::ChildVariant0(c),
///             SimilarRoot::ChildVariant1(c) => Self::ChildVariant1(c),
///             SimilarRoot::ChildVariant2(c) => Self::ChildVariant2(c),
///             SimilarRoot::ChildVariant3(c) => Self::ChildVariant3(c),
///             SimilarRoot::ChildVariant4(c) => Self::ChildVariant4(c),
///         }
///     }
/// }
/// ```
/// and changes it into:
/// ```
/// #![feature(more_qualified_paths)]
/// use treeerror::map_enum;
///
/// struct Child0;
/// struct Child1;
/// struct Child2;
/// struct Child3;
/// struct Child4;
///
/// enum Root {
///     ChildVariant0(Child0),
///     ChildVariant1(Child1),
///     ChildVariant2(Child2),
///     ChildVariant3(Child3),
///     ChildVariant4(Child4),
/// }
///
/// enum SimilarRoot {
///     ChildVariant0(Child0),
///     ChildVariant1(Child1),
///     ChildVariant2(Child2),
///     ChildVariant3(Child3),
///     ChildVariant4(Child4),
/// }
///
/// map_enum!(SimilarRoot > Root {
///     ChildVariant0,
///     ChildVariant1,
///     ChildVariant2,
///     ChildVariant3,
///     ChildVariant4,
/// });
/// ```
///
/// As well as changing things like:
/// ```
/// struct Child0;
/// struct Child1;
///
/// enum Root {
///     ChildVariant0Rename(Child0, Child1),
///     ChildVariant1(Child1, Child0),
/// }
///
/// enum SimilarRoot {
///     ChildVariant0(Child0, Child1),
///     ChildVariant1(Child1, Child0),
/// }
///
/// impl From<SimilarRoot> for Root {
///     fn from(r: SimilarRoot) -> Self {
///         match r {
///             SimilarRoot::ChildVariant0(a, b) => Self::ChildVariant0Rename(a, b),
///             SimilarRoot::ChildVariant1(a, b) => Self::ChildVariant1(a, b),
///         }
///     }
/// }
/// ```
/// into:
/// ```
/// #![feature(more_qualified_paths)]
/// use treeerror::map_enum;
///
/// struct Child0;
/// struct Child1;
///
/// enum Root {
///     ChildVariant0Rename(Child0, Child1),
///     ChildVariant1(Child1, Child0),
/// }
///
/// enum SimilarRoot {
///     ChildVariant0(Child0, Child1),
///     ChildVariant1(Child1, Child0),
/// }
///
/// map_enum!(SimilarRoot > Root {
///     ChildVariant0 > ChildVariant0Rename = (a, b),
///     ChildVariant1 = (a, b),
/// });
/// ```
///
/// This is especially useful when there are multiple external errors that all need to be
/// mapped onto the same error.
/// ```
/// #![feature(more_qualified_paths)]
/// use treeerror::map_enum;
///
/// #[derive(Debug)]
/// pub struct WebError;
///
/// mod suberror0 {
///     #[derive(Debug)]
///     pub(super) struct MemoryError;
///     #[derive(Debug)]
///     pub(super) enum E {
///         NotFound,
///         Web(super::WebError),
///         Memory(MemoryError),
///     }
/// }
///
/// mod suberror1 {
///     #[derive(Debug)]
///     pub(super) struct MemoryError;
///     #[derive(Debug)]
///     pub(super) enum E {
///         Web(super::WebError),
///         Memory(MemoryError),
///         WeirdInternalErrorThatShouldNotBeSurfaced,
///     }
/// }
///
/// enum SharedError {
///     NotFound,
///     Web(WebError),
///     Memory0(suberror0::MemoryError),
///     Memory1(suberror1::MemoryError),
/// }
///
/// map_enum!(suberror0::E > SharedError {
///     @unit NotFound,
///     Web,
///     Memory > Memory0,
/// });
///
/// map_enum!(suberror1::E > SharedError {
///     Web,
///     Memory > Memory1,
/// } |e| {
///     panic!("this should not happen... {e:?}")
/// });
///
/// fn main() {}
///
/// ```
#[macro_export]
macro_rules! map_enum {
    // TODO Add support for specifying "dropping out" of some identities.
    ($from:path > $to:path {
        $($(@$m:ident)* $match:ident $(> $wrap:ident)? $(= ($($p:ident),+ $(,)?))? $($blk:block)?),+ $(,)?
    } $($(|$e:ident|)? $catch:block)?) => {
        impl From<$from> for $to {
            fn from(e: $from) -> Self {
                match e {
                    $(map_enum!(@coerce pat map_enum!(
                        @invocation pat
                        (<$from>::$match)
                        __some_tok
                        $(@$m)*
                        ($($($p),+)?)
                    )) => {
                        map_enum!(
                            @invocation expr
                            (map_enum!(@unwrap_opt $($wrap)? $match (<$to>::)))
                            __some_tok
                            $(@$m)*
                            ($($($p),+)?)
                            $($blk)?
                        )
                    })+
                    $(e => {
                        $(let $e = e;)?
                        $catch
                    })?
                }
            }
        }
    };

    (@invocation pat ($($path:tt)+) $escaped:ident @unit ($($tail:tt)*)) => (
        $($path)+
    );
    (@invocation pat ($($path:tt)+) $escaped:ident $(@$m:ident)* ()) => (
        $($path)+ ($escaped)
    );
    (@invocation pat ($($path:tt)+) $escaped:ident $(@$m:ident)* ($($tail:tt)+)) => (
        $($path)+ ($($tail)+)
    );

    (@invocation expr ($($path:tt)+) $escaped:ident @unit ($($tail:tt)*)) => (
        $($path)+
    );
    (@invocation expr ($($path:tt)+) $escaped:ident @flatten ()) => (
        $escaped
    );
    (@invocation expr ($($path:tt)+) $escaped:ident @flatten @conv ()) => (
        $escaped.into()
    );
    (@invocation expr ($($path:tt)+) $escaped:ident @conv @flatten ()) => (
        $escaped.into()
    );
    (@invocation expr ($($path:tt)+) $escaped:ident @conv ()) => (
        $($path)+ ($escaped.into())
    );
    (@invocation expr ($($path:tt)+) $escaped:ident @conv ($($tail:tt)+)) => (
        map_enum!(@paramlist ($($path)+) $($tail)+)
    );
    (@invocation expr ($($path:tt)+) $escaped:ident $(@$m:ident)* ()) => (
        $($path)+ ($escaped)
    );
    (@invocation expr ($($path:tt)+) $escaped:ident $(@$m:ident)* ($($tail:tt)+)) => (
        $($path)+ ($($tail)+)
    );
    (@invocation expr ($($path:tt)+) $escaped:ident $(@$m:ident)* ($($tail:tt)*) $blk:block) => (
        $blk
    );

    (@paramlist ($($path:tt)+) $p0:ident $(,)?) => (
        $($path)+ ($p0.into())
    );
    (@paramlist ($($path:tt)+) $p0:ident, $p1:ident $(,)?) => (
        $($path)+ ($p0.into(), $p1.into())
    );
    (@paramlist ($($path:tt)+) $p0:ident, $p1:ident, $p2:ident $(,)?) => (
        $($path)+ ($p0.into(), $p1.into(), $p2.into())
    );
    (@paramlist ($($path:tt)+) $p0:ident, $p1:ident, $p2:ident, $p3:ident $(,)?) => (
        $($path)+ ($p0.into(), $p1.into(), $p2.into(), $p3.into())
    );

    (@coerce pat $stuff:pat) => ($stuff);
    (@coerce exprlist ($($stuff:expr),*)) => ($($stuff),*);

    (@unwrap_opt $opt:ident $base:ident ($($tail:tt)*)) => ($($tail)* $opt);
    (@unwrap_opt $base:ident ($($tail:tt)*)) => ($($tail)* $base);
}

#[cfg(test)]
mod test {
    macro_rules! test_types {
        ($sub:ident, $full:ident) => {
            #[allow(dead_code)]
            #[derive(Debug, Clone, PartialEq, Eq, Hash)]
            enum $sub {
                I(i32),
                S(String),
                U(u64),
                R(&'static str),
                M(i32, u64),
                Unit,
            }
            #[allow(dead_code)]
            #[derive(Debug, Clone, PartialEq, Eq, Hash)]
            enum $full {
                I(i32),
                S(String),
                U(u64),
                Ra(&'static str),
                Ma(i32, u64),
                Ib(i32),
                Sb(String),
                Ub(u64),
                Rb(&'static str),
                Mb(i32, u64),
                Unit,
            }
        };
    }

    mod invocation {
        test_types!(Submap, Fullmap);

        #[test]
        fn test_expr_unit() {
            let a = map_enum!(@invocation expr (Submap::Unit) __ignored @unit ());
            assert_eq!(a, Submap::Unit, "escaped identifier to be used");
        }

        #[test]
        fn test_expr_single() {
            let sample = 0u64;
            let a = map_enum!(@invocation expr (Submap::U) sample ());
            assert_eq!(a, Submap::U(0), "escaped identifier to be used");
        }

        #[test]
        fn test_expr_multi() {
            let a = 0i32;
            let b = 2u64;
            let a1 = map_enum!(@invocation expr (Submap::M) __ignored (a, b));
            assert_eq!(a1, Submap::M(0, 2), "escaped identifier to be used");
        }

        #[test]
        fn test_expr_single_convert() {
            let sample = 0u32;
            let a = map_enum!(@invocation expr (Submap::U) sample @conv ());
            assert_eq!(a, Submap::U(0), "escaped identifier to be used");
        }

        #[test]
        fn test_expr_multi_convert() {
            let a = 0i16;
            let b = 2u32;
            let a1 = map_enum!(@invocation expr (Submap::M) __ignored @conv (a, b));
            assert_eq!(a1, Submap::M(0, 2), "escaped identifier to be used");
        }

        #[test]
        fn test_pattern_unit() {
            let s = Submap::Unit;
            match s {
                map_enum!(@invocation pat (Submap::Unit) _a @unit ()) => {
                    return;
                },
                _ => {
                    unimplemented!("`s` should get matched in the previous line.");
                },
            }
        }

        #[test]
        fn test_pattern_single() {
            let s = Submap::I(0);
            match s {
                map_enum!(@invocation pat (Submap::I) a ()) => {
                    assert_eq!(a, 0, "Macro to properly extract the value");
                    return;
                },
                _ => {
                    unimplemented!("`s` should get matched in the previous line.");
                },
            }
        }

        #[test]
        fn test_pattern_multi() {
            let s = Submap::M(0, 1);
            match s {
                map_enum!(@invocation pat (Submap::M) a (a, b)) => {
                    assert_eq!(a, 0, "Macro to properly extract the value");
                    assert_eq!(b, 1, "Macro to properly extract the value");
                    return;
                },
                _ => {},
            };
            unimplemented!("`s` should get matched in the previous line.");
        }
    }

    mod simple {
        test_types!(Submap, Fullmap);

        map_enum!(Submap > Fullmap {
            I > Ib,
        } |_ignored| {
            Fullmap::Unit
        });
    }

    mod alltogether {
        test_types!(S, F);
        map_enum!(S > F {
            I,
            S,
            U > Ub,
            R > Ra,
            M > Ma = (a, b),
            @unit Unit,
        });
    }
}
