//! This crate allows for the creation
//! of type-level values. Any value can
//! be type-level as long as it can be
//! initialized as a constant. All
//! type-level values implement the
//! TypeVal trait, and so can be provided
//! as type parameters.

/// A trait implemented by type-level
/// values.
pub trait TypeVal<T> {
    /// The value held.
    const VAL: T;
}

/// This macro is used to implement the
/// TypeVal trait. Any number of values
/// can be initialized with a single
/// invocation, but every value in a
/// single invocation must be either
/// public or private. Attributes to
/// be applied to items in a block,
/// including doc comments, should go
/// above their targets.
///
/// ## Example
/// 
/// ```rust
/// // Private
/// def_type_val! {
///     type One: i32 = 1;
///     #[derive(Clone, Copy)]
///     type True: bool = true;
/// }
///
/// // Public
/// def_type_val! {
///     /// Negative one
///     pub type NegOne: i32 = -1;
///     pub type False: bool = false;
/// }
/// ```
#[macro_export]
macro_rules! def_type_val {
    {$($(#[$attr:meta])* type $name:ident: $type:ident = $value:expr;)+} => {
        $(
            $(#[$attr])*
            struct $name;
            
            impl $crate::TypeVal<$type> for $name {
                const VAL: $type = $value;
            }
        )+
    };
    {$($(#[$attr:meta])* pub type $name:ident: $type:ident = $value:expr;)+} => {
        $(
            $(#[$attr])*
            pub struct $name;
            
            impl $crate::TypeVal<$type> for $name {
                const VAL: $type = $value;
            }
        )+
    };
}
