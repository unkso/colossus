use devise::{Diagnostic, FromMeta, SpanWrapped, Spanned};
use proc_macro2_diagnostics::SpanDiagnosticExt;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

const ONE_SERVICE_FIELD: &str = "missing `#[service(alias = \"alias\")]` attribute";
const INVALID_ALIAS: &str = "value in `#[service(alias = \"value\")]` is not a valid ident";

#[derive(FromMeta, Debug)]
pub struct ServiceAttr {
    pub alias: SpanWrapped<syn::Expr>,
}

impl ServiceAttr {
    const NAME: &'static str = "service";
}

pub fn derive_attachable_service(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let result = try_derive_attachable_service(parse_macro_input!(input as DeriveInput));

    match result {
        Ok(tokens) => tokens.into(),
        Err(e) => e.emit_as_item_tokens().into(),
    }
}

fn try_derive_attachable_service(
    input: DeriveInput,
) -> Result<proc_macro2::TokenStream, Diagnostic> {
    let attr = ServiceAttr::one_from_attrs(ServiceAttr::NAME, &input.attrs)?
        .ok_or_else(|| input.span().error(ONE_SERVICE_FIELD))?;

    let expr = &attr.alias.value;
    let type_alias = match expr {
        syn::Expr::Path(..) => quote!(#expr),
        _ => return Err(input.span().error(INVALID_ALIAS)),
    };

    let fairing_name = format!("'{}' Service", type_alias);

    Ok(quote! {
        impl ::libservices::AttachableService for #type_alias {
            type Service = Self;

            fn init() -> ::libservices::Initializer<Self> {
                ::libservices::Initializer::with_name(#fairing_name)
            }
        }

        #[::rocket::async_trait]
        impl<'r> ::rocket::request::FromRequest<'r> for &'r #type_alias {
            type Error = ();

            async fn from_request(
                req: &'r ::rocket::request::Request<'_>
            ) -> ::rocket::request::Outcome<Self, Self::Error> {
                match req.rocket().state::<#type_alias>() {
                    Some(service) => ::rocket::outcome::Outcome::Success(service),
                    None => ::rocket::outcome::Outcome::Failure((
                        ::rocket::http::Status::InternalServerError,
                        (),
                    )),
                }
            }
        }

        impl ::rocket::Sentinel for #type_alias {
            fn abort(rocket: &::rocket::Rocket<::rocket::Ignite>) -> bool {
                rocket.state::<Self>().is_none()
            }
        }
    })
}
