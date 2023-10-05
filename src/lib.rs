use proc_macro::TokenStream;
use quote::{ToTokens,quote};
use syn::{parse_macro_input, DeriveInput, Data};
extern crate proc_macro;

macro_rules! num_traits {
    ($name:expr, $arms:expr, [ $($num:tt),* ]) => {
        {
            let mut code = String::new();
            $(code += &num_trait!($name,$arms,$num);)*
            code
        }
    };
}
macro_rules! num_trait {
    ($name:expr, $arms:expr, $num:tt) => {
        format! {
            r#"
                impl From<{2}> for {0} {{
                    fn from(value:{2}) -> Self {{
                        match value {{
                            {1}
                            _ => panic!("Failed convertion from {{}}",value)
                        }}
                    }}
                }}
            "#,
            $name,
            $arms,
            quote!($num),
        }
    };
}
#[proc_macro_derive(FromNum)]
pub fn derive_builder(input:TokenStream) -> TokenStream {
    let inp = parse_macro_input!(input as DeriveInput);
    let name = inp.ident.to_string();
    match inp.data {
        Data::Enum(data) => {
            let mut arms = format!("");
            let mut count = 0;
            for var in data.variants.iter() {
                if let Some((_,expr)) = var.discriminant.clone() {
                    count = parse_with_prefix(&expr.into_token_stream().to_string());
                };
                arms += &format!{
                    "{} => Self::{},",
                    count,
                    var.ident.to_string()
                };
                count += 1;
            }
            let code = num_traits!(name,arms,[i8,i16,i32,i64,isize,u8,u16,u32,u64,usize]);
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