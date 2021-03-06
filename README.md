Dead Simple Rust Quasi-Quoting
==============================

[![Build Status](https://api.travis-ci.org/dtolnay/quote.svg?branch=master)](https://travis-ci.org/dtolnay/quote)
[![Latest Version](https://img.shields.io/crates/v/quote.svg)](https://crates.io/crates/quote)
[![Rust Documentation](https://img.shields.io/crates/v/quote.svg?label=rustdoc)](https://dtolnay.github.io/quote/quote/)

Quasi-quoting without a Syntex dependency, intended for use with [Macros
1.1](https://github.com/rust-lang/rfcs/blob/master/text/1681-macros-1.1.md).

```toml
[dependencies]
quote = "0.2"
```

```rust
#[macro_use]
extern crate quote;
```

Interpolation is done with `#var`:

```rust
let tokens = quote! {
    struct SerializeWith #generics #where_clause {
        value: &'a #field_ty,
        phantom: ::std::marker::PhantomData<#item_ty>,
    }

    impl #generics serde::Serialize for SerializeWith #generics #where_clause {
        fn serialize<S>(&self, s: &mut S) -> Result<(), S::Error>
            where S: serde::Serializer
        {
            #path(self.value, s)
        }
    }

    SerializeWith {
        value: #value,
        phantom: ::std::marker::PhantomData::<#item_ty>,
    }
};
```

Repetition is done in one of three ways, similar to `macro_rules!` repetitions:

- `#(var)*` - no separators
- `#(var),*` - the character before the asterisk is used as a separator
- `#(var;)*` - the character before the close paren is used as a terminator

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
