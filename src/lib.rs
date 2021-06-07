use actix_web::http::StatusCode;
use anyhow::{anyhow, bail, Context};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

struct ParsedVariant {
    status_code: StatusCode,
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

        let attr = variant
            .attrs
            .iter()
            .find(|a| a.path.is_ident("status_code"))
            .ok_or(anyhow!(
                "Attribute #[status_code] is required on each variant"
            ))?;

        let litint = attr
            .parse_args::<syn::LitInt>()
            .context("Status code value must be an integer literal")?;

        let status_code = litint
            .base10_parse::<StatusCode>()
            .context("Status code integer must be parsed into StatusCode")?;

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

        parsed.push(ParsedVariant {
            status_code,
            with_body,
            ident,
        });
    }

    Ok((input.ident, parsed))
}

fn render(ident: Ident, variants: Vec<ParsedVariant>) -> TokenStream {
    let vars = variants.into_iter().map(|parsed| {
        let code = parsed.status_code.as_u16();
        let name = parsed.ident;

        if parsed.with_body {
            quote! {
                #ident::#name(body) => actix_web::HttpResponse::build(actix_web::http::StatusCode::from_u16(#code).unwrap()).json(body),
            }
        } else {
            quote! {
                #ident::#name => actix_web::HttpResponse::build(actix_web::http::StatusCode::from_u16(#code).unwrap()).finish(),
            }
        }
    });

    TokenStream::from(quote! {
        impl actix_web::Responder for #ident {
            type Future = futures::future::Ready<Result<actix_web::HttpResponse, Self::Error>>;
            type Error = ();

            fn respond_to(self, req: &actix_web::HttpRequest) -> Self::Future {
                futures::future::ready(Ok(match self {
                    #(#vars)*
                }))
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
