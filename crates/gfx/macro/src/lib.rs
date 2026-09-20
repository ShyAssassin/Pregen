mod overtrait;

use proc_macro::TokenStream;

#[proc_macro]
// Implement trait based overloading for a type
pub fn overtrait(item: TokenStream) -> TokenStream {
    return overtrait::overtrait_impl(item);
}
