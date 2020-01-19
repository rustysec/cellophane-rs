extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(HasPointer)]
pub fn derive_get_ptr(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl HasPointer for #name {
            fn new() -> Self {
                Self(std::ptr::null_mut())
            }

            fn from_ptr(p: *mut c_void) -> Self {
                Self(p)
            }

            fn ptr(&self) -> *const c_void {
                self.0
            }
            fn mut_ptr(&mut self) -> *mut c_void {
                self.0
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn freeing(attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let input_name = &input.ident;
    let freeing = parse_macro_input!(attr as syn::TypePath);

    let expanded = quote! {
        #[derive(HasPointer)]
        #input

        impl Drop for #input_name {
            fn drop(&mut self) {
                if !self.0.is_null() {
                    unsafe {
                        #freeing(self.0);
                    }
                }
            }
        }
    };

    TokenStream::from(expanded)
}
