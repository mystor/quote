extern crate proc_macro;

#[cfg(use_quote_macros)]
mod detail;

#[cfg(use_quote_macros)]
#[proc_macro]
pub fn quote_one_token_func(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    detail::quote_one_token_func(item)
}
