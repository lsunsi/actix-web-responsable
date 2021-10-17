use anyhow::bail;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

struct ParsedVariant {
    with_body: bool,
    ident: Ident,
}

fn parse(input: DeriveInput) -> anyhow::Result<(Ident, Vec<ParsedVariant>)> {
    let data = match input.data {
        syn::Data::Struct(_) | syn::Data::Union(_) => {
            bail!("Only enums are supported (received struct or union)")
        }
        syn::Data::Enum(data) => data,
    };

    let mut parsed = vec![];

    for variant in data.variants {
        let ident = variant.ident;

        let with_body = match variant.fields {
            syn::Fields::Unnamed(fs) if fs.unnamed.len() == 1 => true,
            syn::Fields::Unnamed(_) => {
                bail!("Only enum variants with zero or one arguments are supported")
            }
            syn::Fields::Named(_) => {
                bail!("Only tuple enum variant is supported (received struct enum variant)")
            }
            syn::Fields::Unit => false,
        };

        parsed.push(ParsedVariant { with_body, ident });
    }

    Ok((input.ident, parsed))
}

fn render(ident: Ident, variants: Vec<ParsedVariant>) -> TokenStream {
    let vars = variants.into_iter().map(|parsed| {
        let name = parsed.ident;

        if parsed.with_body {
            quote! {
                #ident::#name(body) => actix_web::HttpResponse::#name().json(body),
            }
        } else {
            quote! {
                #ident::#name => actix_web::HttpResponse::#name().finish(),
            }
        }
    });

    TokenStream::from(quote! {
        impl actix_web::Responder for #ident {
            fn respond_to(self, req: &actix_web::HttpRequest) -> actix_web::HttpResponse {
                match self {
                    #(#vars)*
                }
            }
        }
    })
}

#[proc_macro_derive(Responder, attributes(status_code))]
pub fn derive_responsable_responder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let (ident, variants) = parse(input).expect("Failed parsing of Responsable");
    render(ident, variants)
}
