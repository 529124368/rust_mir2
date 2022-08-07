extern crate proc_macro;
use crate::proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse_macro_input, DeriveInput};

#[proc_macro_derive(lszMacro)]
pub fn zimu_macro_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let gen = quote! (
        impl Deref for #name {
            type Target = ManBase;
            fn deref(&self) -> &ManBase {
                &self.man
            }
        }

        impl DerefMut for #name {
            fn deref_mut(&mut self) -> &mut ManBase {
                &mut self.man
            }
        }
    );
    gen.into()
}
