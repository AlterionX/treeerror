#![feature(more_qualified_paths)]

//! This crate is intended to provide some convenience enums for mapping enums from one type to
//! another. This is intended as an expansion of `thiserror` that has more compile time mapping
//! between sets of enums as well as shortcutting `From` implementations.
//!
//! This is how to use wrapped errors.
//!
//! ```
//! #![feature(more_qualified_paths)]
//!
//! use error_rules::{from_many, from};
//!
//! #[derive(PartialEq, Debug)]
//! enum Root {
//!     A(A),
//! }
//! #[derive(PartialEq, Debug)]
//! enum A {
//!     Alpha(Alpha),
//! }
//! #[derive(PartialEq, Debug)]
//! enum Alpha {
//!     One(One),
//! }
//! #[derive(PartialEq, Debug)]
//! enum One {
//!     Child(Child),
//! }
//! #[derive(PartialEq, Debug)]
//! struct Child;
//!
//! from_many!(Root: A, A, Alpha, One, Child);
//! from_many!(A: Alpha, Alpha, One, Child);
//! from_many!(Alpha: One, One, Child);
//! from!(One = Child(Child));
//!
//! const MSG: &'static str = "hello from below";
//!
//! fn root_error() -> Result<(), Root> {
//!   Err(Child)?;
//!
//!   unreachable!("error should have returned earlier");
//! }
//!
//! assert_eq!(
//!   Err(Root::A(A::Alpha(Alpha::One(One::Child(Child))))),
//!   root_error()
//! );
//! ```
//!
//! You'll notice that there are many `from_many` call here -- automatically generating this
//! tree is not yet complete and in progress.
//!
//! We can also handle "flatter" error structures where, instead of nesting these enums by
//! wrapping them, we destructure the internal values instead.
//!
//! There are several ways to use this as well, but here's the most basic use case.
//!
//! ```
//! #![feature(more_qualified_paths)]
//!
//! use error_rules::map_enum;
//!
//! #[derive(PartialEq, Eq, Debug)]
//! pub enum ParentError {
//!   Child(&'static str),
//! }
//!
//! #[derive(PartialEq, Eq, Debug)]
//! pub enum ChildError {
//!   SomeError(&'static str),
//! }
//!
//! map_enum!(ChildError > ParentError {
//!   SomeError > Child
//! });
//!
//! const MSG: &'static str = "hello there";
//!
//! fn parent_error() -> Result<(), ParentError> {
//!   Err(ChildError::SomeError(MSG))?;
//!
//!   unreachable!("error should have returned earlier");
//! }
//!
//! assert_eq!(
//!   Err(ParentError::Child(MSG)),
//!   parent_error(),
//! );
//! ```
//!
//! Some things that remain: combining all of this in a nice interface to autogenerate the
//! error chains, as well as packaging all of this stuff in a way to enable better error
//! messages during compilation. This potentially includes a proc macro rewrite to enable
//! better error messages and configurability.

mod shared;

mod simple;
mod chain;
mod mapping;
mod tree;

use std::future::Future;

pub use simple::*;
pub use chain::*;
pub use mapping::*;
// pub use tree::*;

// TODO Remove once https://github.com/rust-lang/rust/issues/102211 is resolved.
pub fn assert_send<'u, R: Send>(fut: impl 'u + Send + Future<Output = R>) -> impl 'u + Send + Future<Output = R> {
    fut
}

#[cfg(test)]
mod test {
    mod test_skips {
        use crate::*;

        enum Root {
            A(A),
        }
        enum A {
            Alpha(Alpha),
        }
        enum Alpha {
            One(One),
        }
        enum One {
            Child(Child),
        }
        struct Child;

        from_many!(Root : A, A, Alpha, One, Child);
        from_many!(A: Alpha, Alpha, One, Child);
        from_many!(Alpha: One, One, Child);
        from!(One = Child(Child));

        #[test]
        fn test_from_impls() {
            let _: One = Child.into();
            let _: Alpha = One::from(Child).into();
            let _: A = Alpha::from(Child).into();
            let _: Root = Root::from(Child).into();

            let _: Alpha = Child.into();
            let _: A = Child.into();
            let _: Root = Child.into();
        }
    }

    mod all_together {
        use crate::*;

        enum Root {
            A(A),
        }
        enum A {
            Alpha(Alpha),
        }
        enum Alpha {
            One(One),
        }
        enum One {
            Child(Child),
        }
        struct Child;

        from_chain!(Root : A, A : Alpha, Alpha : One, One : Child, Child);

        #[test]
        fn test_from_impls() {
            let _: One = Child.into();
            let _: Alpha = One::from(Child).into();
            let _: A = Alpha::from(Child).into();
            let _: Root = Root::from(Child).into();

            let _: Alpha = Child.into();
            let _: A = Child.into();
            let _: Root = Child.into();
        }
    }

    // from_chain!(Child, One:Child, Alpha:One, A:Alpha, Root:A);
}
