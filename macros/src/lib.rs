use once_cell::sync::Lazy;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use regex::Regex;
use syn::{Error, Ident, ItemFn, Lit, Meta, MetaNameValue};

#[proc_macro_attribute]
pub fn challenge(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let function = syn::parse_macro_input!(item as ItemFn);

    let info = match parse_challenge(&function) {
        Ok(i) => i,
        Err(e) => return e.to_compile_error().into(),
    };

    quote! (
        #function

        inventory::submit! {
            #info
        }
    )
    .into()
}

fn parse_challenge(function: &ItemFn) -> Result<ChallengeInfo, Error> {
    let function_name = function.sig.ident.clone();

    let doc_attr = function
        .attrs
        .iter()
        .filter_map(|attr| match attr.parse_meta() {
            Ok(Meta::NameValue(MetaNameValue {
                path,
                lit: Lit::Str(s),
                ..
            })) if path.is_ident("doc") => Some(s.value()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n");

    let (day, name, description) = parse_doc_comment(&doc_attr)?;

    Ok(ChallengeInfo {
        number: day.to_string(),
        name: name.to_string(),
        description: description.to_string(),
        examples: Vec::new(),
        function_name,
    })
}

fn parse_doc_comment(docs: &str) -> Result<(&str, &str, &str), Error> {
    static PATTERN: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"(?i)day ([\d\w]+)\s*:\s*([\w \d]+)").unwrap()
    });

    if docs.is_empty() {
        return Err(Error::new(
            Span::call_site(),
            "Challenges must use doc-comments for their name and description",
        ));
    }

    let captures = match PATTERN.captures(docs) {
        Some(c) => c,
        None => {
            return Err(Error::new(
                Span::call_site(),
                r#"Unable to determine the challenge name and day. Expected something like "Day 1: Report Repair""#,
            ))
        }
    };

    let day = captures.get(1).unwrap().as_str();
    let name = captures.get(2).unwrap().as_str();

    // TODO: Use pulldown-cmark to extract the description section
    let description = "";

    Ok((day, name, description))
}

#[derive(Debug, Clone)]
struct ChallengeInfo {
    number: String,
    name: String,
    description: String,
    examples: Vec<(String, String)>,
    function_name: Ident,
}

impl ToTokens for ChallengeInfo {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ChallengeInfo {
            number,
            name,
            description,
            examples,
            function_name,
        } = self;

        let examples = examples.iter().map(|(ref input, ref expected)| {
            quote! {
                aoc_core::Example {
                    input: #input,
                    expected: #expected,
                }
            }
        });

        let got = quote! {
            aoc_core::Challenge {
                number: #number,
                name: #name,
                description: #description,
                examples: &[ #( #examples => #examples )*],
                solve: |input| -> Result<String, anyhow::Error> {
                    let input  = input.parse()?;
                    let result = #function_name(input)?;

                    Ok(result.to_string())
                },
            }
        };

        tokens.extend(got);
    }
}
