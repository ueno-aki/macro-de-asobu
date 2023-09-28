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
                    count = parse_with_prefix(&expr.into_token_stream().to_string());
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

fn parse_with_prefix(s: &str) -> usize {
    let radix = match s {
        s if s.starts_with("0b") => 2,
        s if s.starts_with("0o") => 8,
        s if s.starts_with("0x") => 16,
        _ => {
            return usize::from_str_radix(s, 10).unwrap();
        },
    };
    usize::from_str_radix(&s[2..], radix).unwrap()
}