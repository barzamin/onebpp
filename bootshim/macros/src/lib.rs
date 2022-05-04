use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, ItemFn, ReturnType, Type, Visibility};

#[proc_macro_attribute]
pub fn entry(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut f = parse_macro_input!(input as ItemFn);

    let valid_sig = f.sig.constness.is_none()
        && f.vis == Visibility::Inherited
        && f.sig.abi.is_none()
        && f.sig.inputs.is_empty()
        && f.sig.asyncness.is_none()
        && f.sig.generics.params.is_empty()
        && f.sig.generics.where_clause.is_none()
        && f.sig.variadic.is_none()
        && match f.sig.output {
            ReturnType::Default => false,
            ReturnType::Type(_, ref ty) => matches!(**ty, Type::Never(_)),
        };

    if !valid_sig {
        return syn::parse::Error::new(
            f.span(),
            "`#[entry]` function must have signature `[unsafe] fn() -> !`",
        )
        .to_compile_error()
        .into();
    }

    if !args.is_empty() {
        return syn::parse::Error::new(f.span(), "#[entry] does not accept arguments")
            .to_compile_error()
            .into();
    }

    f.sig.ident = Ident::new(&format!("__bshimmed_{}", f.sig.ident), Span::call_site());

    // extension point for any additional preprep
    let ident = &f.sig.ident;
    let thunk_ident = Ident::new(&format!("{}_thunk", ident), Span::call_site());

    let attrs = &f.attrs;
    quote!(
        #(#attrs)*
        #[doc(hidden)]
        #[export_name = "main"]
        pub unsafe extern "C" fn #thunk_ident() {
            #ident()
        }

        #f
    )
    .into()
}
