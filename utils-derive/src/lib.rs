use eyre::{eyre, Result};
use proc_macro::{TokenStream, TokenTree};
use quote::quote;
use syn::{parse_macro_input, parse_quote, ItemFn};

#[proc_macro_attribute]
pub fn aoc(attr: TokenStream, item: TokenStream) -> TokenStream {
    let (d, p) = parse_attr(attr).unwrap();
    let mut func = parse_macro_input!(item as ItemFn);
    func.vis = parse_quote!(pub);
    let gen = if p == 1 {
        func.sig.ident = parse_quote!(solve_one);
        quote! {

            mod inner_one {
                use crate::*;
                use crate::inner_two::*;
                use eyre::Result;

                #func
            }
            fn main() -> eyre::Result<()> {
                use utils::Parser;
                let args = utils::Args::parse();

                let input = args.get_input(#d)?;

                if args.run_one() {
                    println!("part one:\n{}", inner_one::solve_one(&input)?);
                }
                if args.run_two() {
                    println!("part two:\n{}", inner_two::solve_two(&input)?);
                }

                Ok(())
            }
        }
    } else {
        func.sig.ident = parse_quote!(solve_two);
        quote! {
            mod inner_two {
                use crate::*;
                use crate::inner_one::*;
                use eyre::Result;

                #func
            }
        }
    };
    gen.into()
}

fn parse_attr(attr: TokenStream) -> Result<(i32, i32)> {
    let mut i = attr.into_iter().filter_map(|t| {
        if let TokenTree::Ident(_) = t {
            Some(t.to_string())
        } else {
            None
        }
    });
    let d = i
        .next()
        .ok_or(eyre!("expecting 'dayX' first attribute"))
        .and_then(|d| {
            d.strip_prefix("day")
                .ok_or(eyre!("expecting 'dayX' attribute first, got {}", d))?
                .parse::<i32>()
                .map_err(|e| {
                    eyre!(
                        "expecting 'dayX', could not parse day number in {} - {}",
                        d,
                        e
                    )
                })
        })?;
    let p = i
        .next()
        .ok_or(eyre!("expecting 'part<1|2>' second attribute"))
        .and_then(|p| {
            p.strip_prefix("part")
                .ok_or(eyre!("expecting 'part<1|2>' second attribute, got {}", d))?
                .parse::<i32>()
                .map_err(|e| {
                    eyre!(
                        "expecting 'part<1|2>', could not parse part number in {} - {}",
                        d,
                        e
                    )
                })
                .and_then(|num| {
                    if num != 1 && num != 2 {
                        Err(eyre!("expecting part number 1 or 2, got {}", num))
                    } else {
                        Ok(num)
                    }
                })
        })?;
    if let Some(n) = i.next() {
        Err(eyre!("unexpected attr - {}", n))
    } else {
        Ok((d, p))
    }
}
