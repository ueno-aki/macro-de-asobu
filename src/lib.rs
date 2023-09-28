use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, DeriveInput, Data};

extern crate proc_macro;

#[proc_macro_derive(FromNum)]
pub fn derive_builder(input:TokenStream) -> TokenStream {
    let inp = parse_macro_input!(input as DeriveInput);
    let name = inp.ident;
    match inp.data {
        Data::Enum(data) => {
            let mut fns = format!("");
            let mut count = 0;
            for var in data.variants.iter() {
                if let Some((_,expr)) = var.discriminant.clone() {
                    count = expr.into_token_stream().to_string().parse::<usize>().unwrap();
                };
                fns += &format!{
                    "{} => Self::{},",
                    count,
                    var.ident.to_string()
                };
                count += 1;
            }
            let code = format!{
                r#"
                    impl {} {{
                        pub fn from_usize(value:usize) -> Self{{
                            match value {{
                                {}
                                _ => panic!("")
                            }}
                        }}
                    }}
                "#,
                name.to_string(),
                fns
            };
            code.parse().unwrap()
        }
        _ => unimplemented!()
    }
}