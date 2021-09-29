extern crate proc_macro;
extern crate proc_macro2;

use darling::FromMeta;
use syn::{AttributeArgs, ItemFn};
use proc_macro2::{Ident, Span};
use syn::parse_macro_input;
use proc_macro::TokenStream;
use quote::quote;

/// the command args expected from the macro attribute
#[derive(Debug, FromMeta)]
struct Args {
    #[darling(default)]
    help: Option<String>,
}

#[proc_macro_attribute]
pub fn make_command(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(args as AttributeArgs);
    let command_fn = parse_macro_input!(input as ItemFn);
    // to get the ident name
    // We need to replace r# since there is no way to do that from the library
    // https://github.com/dtolnay/syn/issues/478
    let cmd_name = command_fn.sig.ident
        .to_string()
        .trim_start_matches("r#")
        .to_owned();
    // turning the function name into an identifier, that can be embedded in the final tokenstream
        let ident = Ident::new(&format!("{}", cmd_name), Span::call_site());
    let cmd_args = match Args::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors())
        }
    };

    let cmd_help = match cmd_args.help {
        Some(v) => v,
        None => String::from("")
    };

    let output = quote!{
        struct #ident;

        impl<'a> cmd_rs::Command<'a> for #ident {
            fn name(&self) -> &str {
                #cmd_name
            }

            fn help(&self) -> Option<&str> {
                if #cmd_help.is_empty() {
                    return None;
                }
                Some(#cmd_help)
            }

            fn execute(&self, _args: &[&str]) -> cmd_rs::Result<()> {
                #command_fn
                #ident()
            }
        }

        inventory::submit!(cmd_rs::RegisteredCommand(&#ident))
    };
    output.into()
}
