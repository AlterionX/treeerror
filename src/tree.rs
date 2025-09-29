// TODO: Consider how to process tress.
#[macro_export]
macro_rules! treeerror {
    {
        $(
            $(#[$($node_cfg:tt)+])*
            $node:ident $(@$wrapper_modifier:ident)? $({ $($subtree:tt)+ })?
        ),+ $(,)?
    } => {
        $(
            $crate::treeerror! {
                @classes
                $(#[$($node_cfg)+])*
                $node $(@$wrapper_modifier)? $({ $($subtree)+ })?
            }
            $crate::treeerror! {
                @froms
                $node $(@$wrapper_modifier)? $({ $($subtree)+ })?
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
                    $subnode:ident $(@$subnode_modifier:ident)? $({ $($subtree:tt)+ })?
                ),* $(,)?
            })?
        ),+
    } => {
        $(
            // $node decl
            $crate::treeerror! {
                @class
                $(#[$($node_cfg)+])*
                $node $(@$node_modifier)? $({
                    $(
                        $subnode $(@$subnode_modifier)? $({ $($subtree)+ })?
                    ),*
                })?
            }
            // $node children decl
            $($($crate::treeerror! {
                @classes
                $(#[$($subnode_cfg)+])*
                $subnode $(@$subnode_modifier)? $({ $($subtree)+ })?
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
        pub struct $node;
    };
    // Generate enum, needs to munch variant by variant because stupid rules
    {
        @class
        $(#[$($node_cfg:tt)+])*
        $node:ident {
            $(
                $(#[$($subnode_cfg:tt)+])*
                $subnode:ident $(@$modifier:ident)? $({ $($subtree:tt)+ })?
            ),* $(,)?
        }
    } => {
        $crate::treeerror! {
            @enum_class {
                $(#[$($node_cfg)+])*
                $node 
            }
            @variants {
                $($subnode $(@$modifier)?),*
            }
            @processed {}
        }
    };
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
        $node:ident @$some_modifier:ident $({ $($subtree:tt)+ })?
    } => {
        compile_warn!(concat!($node, " was provided an unknown modifier", $some_modifier, ". assuming unit modifier passed"));
        $crate::treeerror! {
            @class
            $(#[$($node_cfg)*])*
            $node @unit
        }
    };
    {
        @class
        $(#[$($node_cfg:tt)+])*
        $node:ident { $($subtree:tt)+ }
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
            $subnode:ident @unit $(,)?
            $($subnode_tail:ident $(@$modifier_tail:ident)?),*
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
                $($subnode_tail $(@$modifier_tail)?),*
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
            $($subnode_tail:ident $(@$modifier_tail:ident)?),*
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
                $($subnode_tail $(@$modifier_tail)?),*
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
            $($subnode_tail:ident $(@$modifier_tail:ident)?),*
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
                $($subnode_tail $(@$modifier_tail)?),*
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
        pub enum $node {
            $($processed)*
        }
    };

    {
        @froms
        $($node:ident {
            $(
                $(#[$($node_cfg:tt)+])*
                $subnode:ident $(@$wrapper_modifier:ident)? $({ $($subtree:tt)+ })?
            ),* $(,)?
        }),* $(,)?
    } => {};

    // AHHHHHH
    // need `$stuff` to be "built" before declaring it as part of the enum because ... reasons
    // (compiler needs each macro to emit "proper" rust code, so we can't emit
    // `pub enum { macro!() }`
    (@force_enum ($($stuff:tt)+) $name:ident) => { pub enum $name { $($stuff)+ } };
    (@coerce_tt $($stuff:tt)+) => { $($stuff)+ };
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
                W3 @unit,
            },
        }
    }

    #[test]
    fn test_derived_traits() {
        assert_eq!(format!("{:?}", Hello::FlatWorld), "FlatWorld");
        assert_eq!(format!("{:?}", World), "World");
    }
}
