// TODO: Consider how to process tress.
#[macro_export]
macro_rules! treeerror {
    {
        $(
            $(#[$($node_cfg:tt)+])*
            $node:ident $(@$wrapper_modifier:ident)? $({ $($subtree:tt)+ })? $(($wrapped:ty))?
        ),+ $(,)?
    } => {
        $(
            $crate::treeerror! {
                @classes
                $(#[$($node_cfg)+])*
                $node $(@$wrapper_modifier)? $({ $($subtree)+ })? $(($wrapped))?
            }

            $crate::treeerror! {
                @froms
                ()
                $node $(@$wrapper_modifier)? $({ $($subtree)+ })? $(($wrapped))?
            }
        )+
    };

    {
        @classes
        $(
            $(#[$($node_cfg:tt)+])*
            $node:ident $(@$node_modifier:ident)? $({
                $(
                    $(#[$($subnode_cfg:tt)+])*
                    $subnode:ident $(@$subnode_modifier:ident)? $({ $($subtree:tt)+ })? $(($subwrapped:ty))?
                ),* $(,)?
            })? $(($wrapped:ty))?
        ),+
    } => {
        $(
            // $node decl
            $crate::treeerror! {
                @class
                $(#[$($node_cfg)+])*
                $node $(@$node_modifier)? $({
                    $(
                        $subnode $(@$subnode_modifier)? $({ $($subtree)+ })? $(($subwrapped))?
                    ),*
                })? $(($wrapped))?
            }
            // $node children decl
            $($($crate::treeerror! {
                @classes
                $(#[$($subnode_cfg)+])*
                $subnode $(@$subnode_modifier)? $({ $($subtree)+ })? $(($subwrapped))?
            })*)?
        )+
    };

    // Shows an error message so we don't get too far without knowing why it broke.
    // Unit struct wrappers
    {
        @class
        $(#[$($node_cfg:tt)+])*
        $node:ident @unit { $($subtree:tt)+ }
    } => {
        compiler_warn!(concat!($node, " was provided a subtree despite it having a unit modifier, please only have one of the two. assuming unit struct"));
        $crate::treeerror! {
            @class
            $(#[$($node_cfg)*])*
            $node @unit
        }
    };
    {
        @class
        $(#[$($node_cfg:tt)+])*
        $node:ident @unit
    } => {
        $(#[$($node_cfg)+])*
        #[allow(dead_code)]
        pub struct $node;
    };
    // Generate enum, needs to munch variant by variant because stupid rules
    {
        @class
        $(#[$($node_cfg:tt)+])*
        $node:ident {
            $(
                $(#[$($subnode_cfg:tt)+])*
                $subnode:ident $(@$modifier:ident)? $({ $($subtree:tt)+ })? $(($subwrapped:ty))?
            ),* $(,)?
        }
    } => {
        $crate::treeerror! {
            @enum_class {
                $(#[$($node_cfg)+])*
                $node 
            }
            @variants {
                $($subnode $(@$modifier)? $(($subwrapped))?),*
            }
            @processed {}
        }
    };
    // For enum variants wrapping explicit types, there's no other class that gets wrapped down
    // here -- ignore!
    {
        @class
        $(#[$($node_cfg:tt)+])*
        $node:ident ($wrapped:ty)
    } => {};
    // Flatunit is handled elsewhere (in enum_class) -- ignore!
    {
        @class
        $(#[$($node_cfg:tt)+])*
        $node:ident @flatunit
    } => {};
    // Final few catchalls in case user put in something weird
    {
        @class
        $(#[$($node_cfg:tt)+])*
        $node:ident @$some_modifier:ident $({ $($subtree:tt)+ })? $(($wrapped:ty))?
    } => {
        $crate::treeerror! {
            @class
            $(#[$($node_cfg)*])*
            $node @unit
        }
    };
    {
        @class
        $(#[$($node_cfg:tt)+])*
        $node:ident { $($subtree:tt)+ } ($wrapped:ty)
    } => {
        compile_error!(concat!(stringify!($node), " couldn't be parsed."));
    };

    // Simple enum generator that munches variant by variant
    // forced to do it because Rust macros have the stupid rules must generate valid expression
    // rule when the token tree is long enough to be parsed.
    //
    // And we need to do this mutual recursion thing because dumbness
    {
        @enum_class {
            $(#[$($node_cfg:tt)+])*
            $node:ident 
        }
        @variants {
            $subnode:ident ($wrapped:ty) $(,)?
            $($subnode_tail:ident $(@$modifier_tail:ident)? $(($wrapped_tail:ty))?),*
        }
        @processed {
            $($processed:tt)*
        }
    } => {
        $crate::treeerror! {
            @enum_class {
                $(#[$($node_cfg)+])*
                $node
            }
            @variants {
                $($subnode_tail $(@$modifier_tail)? $(($wrapped_tail))?),*
            }
            @processed {
                $($processed)*
                $subnode($wrapped),
            }
        }
    };
    {
        @enum_class {
            $(#[$($node_cfg:tt)+])*
            $node:ident 
        }
        @variants {
            $subnode:ident @unit $(,)?
            $($subnode_tail:ident $(@$modifier_tail:ident)? $(($wrapped_tail:ty))?),*
        }
        @processed {
            $($processed:tt)*
        }
    } => {
        $crate::treeerror! {
            @enum_class {
                $(#[$($node_cfg)+])*
                $node
            }
            @variants {
                $($subnode_tail $(@$modifier_tail)? $(($wrapped_tail))?),*
            }
            @processed {
                $($processed)*
                $subnode($subnode),
            }
        }
    };
    {
        @enum_class {
            $(#[$($node_cfg:tt)+])*
            $node:ident
        }
        @variants {
            $subnode:ident @flatunit $(,)?
            $($subnode_tail:ident $(@$modifier_tail:ident)? $(($wrapped_tail:ty))?),*
        }
        @processed {
            $($processed:tt)*
        }
    } => {
        $crate::treeerror! {
            @enum_class {
                $(#[$($node_cfg)+])*
                $node
            }
            @variants {
                $($subnode_tail $(@$modifier_tail)? $(($wrapped_tail))?),*
            }
            @processed {
                $($processed)*
                $subnode,
            }
        }
    };
    {
        @enum_class {
            $(#[$($node_cfg:tt)+])*
            $node:ident
        }
        @variants {
            $subnode:ident $(,)?
            $($subnode_tail:ident $(@$modifier_tail:ident)? $(($wrapped_tail:ty))?),*
        }
        @processed {
            $($processed:tt)*
        }
    } => {
        $crate::treeerror! {
            @enum_class {
                $(#[$($node_cfg)+])*
                $node
            }
            @variants {
                $($subnode_tail $(@$modifier_tail)? $(($wrapped_tail))?),*
            }
            @processed {
                $($processed)*
                $subnode($subnode),
            }
        }
    };
    {
        @enum_class {
            $(#[$($node_cfg:tt)+])*
            $node:ident
        }
        @variants {}
        @processed {
            $($processed:tt)*
        }
    } => {
        $(#[$($node_cfg)+])*
        #[allow(dead_code)]
        pub enum $node {
            $($processed)*
        }
    };

    {
        @froms
        ($($parents:ident),* $(,)?)
    } => {
    };
    // Peel off one by one, needs to be a separate rule due to duplicate comma parsing
    {
        @froms
        ($($parents:ident),* $(,)?)
        $(#[$($node_cfg:tt)+])*
        $node:ident $(@$node_modifier:ident)? $({ $($subtree:tt)+ })? $(($wrapped:ty))? $(,)?
        $($(
            $(#[$($tail_cfg:tt)+])*
            $tail_nodes:ident $(@$tail_modifier:ident)?  $({ $($tail_subtree:tt)+ })? $(($tail_wrapped:ty))?
        ),+ $(,)?)?
    } => {
        $crate::treeerror! {
            @maybe_from_impls $($node_modifier)?
            ($node, $($parents),*)
            $(($wrapped))?
        }
        $crate::treeerror! {
            @froms
            ($node, $($parents),*)
            $($($subtree)+)?
        }
        $crate::treeerror! {
            @froms
            ($($parents),*)
            $($(
                $tail_nodes $(@$tail_modifier)?  $({ $($tail_subtree)+ })? $(($tail_wrapped))?
            ),+)?
        }
    };

    {
        @maybe_from_impls flatunit
        ($($node:ident),* $(,)?)
        $(($wrapped:ty))?
    } => {};
    {
        @maybe_from_impls unit
        ($($node:ident),* $(,)?)
        $(($wrapped:ty))?
    } => {
        $crate::treeerror! {
            @from_impls
            ($($node),*)
            $(($wrapped))?
        }
    };
    {
        @maybe_from_impls
        ($($node:ident),* $(,)?)
        $(($wrapped:ty))?
    } => {
        $crate::treeerror! {
            @from_impls
            ($($node),*)
            $(($wrapped))?
        }
    };

    {
        @from_impls
        ()
        $(($wrapped:ty))?
    } => {};
    {
        @from_impls
        ($goal:ident $(,)?)
        $(($wrapped:ty))?
    } => {};
    {
        @from_impls
        ($node:ident, $goal:ident $(,)?)
        ($wrapped:ty)
    } => {
        $crate::from!($goal = $node($wrapped));
    };
    {
        @from_impls
        ($node:ident, $goal:ident $(,)?)
    } => {
        $crate::from!($goal = $node($node));
    };
    {
        @from_impls
        ($node:ident, $via:ident, $goal:ident $(,)? $($($tail:ident),+ $(,)?)?)
        $(($wrapped:ty))?
    } => {
        $crate::from!($goal = $node > $via);
        $crate::treeerror! {
            @from_impls
            ($node , $via $(, $($tail),+)?)
            $(($wrapped))?
        }
    };
}

#[cfg(test)]
mod test {
    crate::treeerror! {
        #[derive(Debug)]
        Hello {
            #[derive(Debug)]
            World @unit,
            FlatWorld @flatunit,
            #[derive(Debug)]
            OtherWorld {
                #[derive(Debug)]
                W0 @unit,
                #[derive(Debug)]
                W1 @unit,
                #[derive(Debug)]
                W2 @unit,
                #[derive(Debug)]
                W3 @flatunit,
            },
            Terminal(String),
            LifetimeTerminal(&'static str),
            #[derive(Debug)]
            Test {
                A @flatunit,
            },
        }
    }

    #[test]
    fn test_class_derivations() {
        assert_eq!(format!("{:?}", Hello::FlatWorld), "FlatWorld");
        assert_eq!(format!("{:?}", Hello::World(World)), "World(World)");
        assert_eq!(format!("{:?}", Hello::OtherWorld(OtherWorld::W0(W0))), "OtherWorld(W0(W0))");
        assert_eq!(format!("{:?}", Hello::OtherWorld(OtherWorld::W1(W1))), "OtherWorld(W1(W1))");
        assert_eq!(format!("{:?}", Hello::OtherWorld(OtherWorld::W2(W2))), "OtherWorld(W2(W2))");
        assert_eq!(format!("{:?}", Hello::OtherWorld(OtherWorld::W3)), "OtherWorld(W3)");
        assert_eq!(format!("{:?}", Hello::Terminal("hi".to_owned())), "Terminal(\"hi\")");
        assert_eq!(format!("{:?}", Hello::LifetimeTerminal("hi")), "LifetimeTerminal(\"hi\")");
    }

    #[test]
    fn test_from_derivations() {
        assert_eq!(format!("{:?}", Hello::FlatWorld), "FlatWorld");
        assert_eq!(format!("{:?}", Hello::from(World)), "World(World)");
        assert_eq!(format!("{:?}", OtherWorld::from(W0)), "W0(W0)");
        assert_eq!(format!("{:?}", Hello::from(W0)), "OtherWorld(W0(W0))");
        assert_eq!(format!("{:?}", OtherWorld::from(W1)), "W1(W1)");
        assert_eq!(format!("{:?}", Hello::from(W1)), "OtherWorld(W1(W1))");
        assert_eq!(format!("{:?}", OtherWorld::from(W2)), "W2(W2)");
        assert_eq!(format!("{:?}", Hello::from(OtherWorld::W3)), "OtherWorld(W3)");
        assert_eq!(format!("{:?}", Hello::from("hi".to_owned())), "Terminal(\"hi\")");
        assert_eq!(format!("{:?}", Hello::from("hi")), "LifetimeTerminal(\"hi\")");
    }
}
