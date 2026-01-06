use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn mod_init(_attr: TokenStream, item: TokenStream) -> TokenStream {
	let inp = parse_macro_input!(item as ItemFn);
	let fn_name = &inp.sig.ident;

	let expanded = quote! {
		#inp

		#[unsafe(no_mangle)]
		pub fn amodus_mod_init() {
			#fn_name();
		}
	};

	TokenStream::from(expanded)
}
