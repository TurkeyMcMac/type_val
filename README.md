# type_val
This project allows for the creation
of type-level values. Any value can
be type-level as long as it can be
initialized as a constant. All
type-level values implement the
TypeVal trait, and so can be provided
as type parameters.

## TypeVal
A trait implemented by type-level
values. The value of a TypeVal can
be extracted through the `VAL`
associated constant.

## def_type_val
This macro is used to implement the
TypeVal trait. Any number of values
can be initialized with a single
invocation. Items prefixed by `pub`
are public. Attributes to be applied
to items in a block, including doc
comments, should go above their
targets.

## Example

```rust
def_type_val! {
    type One: i32 = 1;
    #[derive(Clone, Copy)]
    type True: bool = true;
    /// Negative one
    pub type NegOne: i32 = -1;
    pub type False: bool = false;
}
```
