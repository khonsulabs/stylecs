use attribute_derive::Attribute;
use manyhow::manyhow;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{DeriveInput, Expr};

#[derive(Attribute, Debug)]
#[attribute(ident = style)]
struct StyleComponent {
    name: Option<Ident>,
    authority: Option<Ident>,
    inherited: Option<bool>,
    merge: Option<Expr>,
}

#[manyhow]
#[proc_macro_derive(StyleComponent, attributes(style))]
pub fn style_component_derive(input: TokenStream) -> manyhow::Result<TokenStream> {
    let DeriveInput {
        attrs,
        ident,
        generics,
        ..
    } = syn::parse2(input)?;
    let StyleComponent {
        name,
        authority,
        inherited,
        merge,
    } = StyleComponent::from_attributes(&attrs)?;

    let name = if let Some(name) = name {
        validate(&name)?
    } else {
        stylecs_shared::pascal_case_to_snake_case(ident.to_string()).map_err(|_| manyhow::error_message!(ident.span(), "An invalid character for a stylecs Identifier was found. A name must be manually provided for this type."))?
    };

    let name = if let Some(authority) = authority {
        let authority = validate(&authority)?;
        quote!(::stylecs::static_name!(#authority, #name))
    } else {
        quote!(::stylecs::static_name!(#name))
    };

    let inherited = inherited.map(|value| {
        quote!(
            fn inherited() -> bool {
                #value
            }
        )
    });
    let merge = merge.map(|expr| {
        quote!(
            fn merge(&mut self, other: &Self) {
                #expr;
            }
        )
    });

    Ok(quote! {
        impl<#generics> stylecs::StyleComponent for #ident<#generics> {
            fn name() -> ::stylecs::Name {
                static NAME: ::stylecs::StaticName = #name;
                NAME.to_name()
            }
            #inherited
            #merge
        }
    })
}

fn validate(name: &Ident) -> manyhow::Result<String> {
    let location = name.span();
    let name = name.to_string();
    stylecs_shared::validate_identifier(&name)
        .map_err(|_| manyhow::error_message!(location, "invalid character in identifier"))?;
    Ok(name)
}
