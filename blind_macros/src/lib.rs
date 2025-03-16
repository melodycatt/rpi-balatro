use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemTrait, Meta, TraitItem, TraitItemFn, DeriveInput, parse_quote};
//a
#[proc_macro_attribute]
pub fn generate_trait_getters(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemTrait);
    let trait_name = &input.ident;

    let mut getter_methods = Vec::new();

    // Scan trait items for #[getter(name: Type)] attributes
    for item in &input.items {
        if let TraitItem::Fn(method) = item {
            for attr in &method.attrs {
                if let Meta::List(meta_list) = &attr.meta {
                    if meta_list.path.is_ident("getter") {
                        let tokens = meta_list.tokens.to_string();
                        if let Some((name, ty)) = parse_getter_attr(&tokens) {
                            let fn_name = syn::Ident::new(&name, method.sig.ident.span());
                            let return_type = match syn::parse_str::<syn::Type>(&ty) {
                                Ok(t) => t,
                                Err(_) => return syn::Error::new_spanned(trait_name, "Invalid type in #[getter]").to_compile_error().into(),
                            };
                            
                            // Convert quoted tokens into a TraitItemFn
                            let getter_fn: TraitItemFn = parse_quote! {
                                fn #fn_name(&self) -> &#return_type;
                            };

                            getter_methods.push(TraitItem::Fn(getter_fn));
                        }
                    }
                }
            }
        }
    }

    // Extend the trait with generated getter methods
    input.items.extend(getter_methods);

    let items = &input.items;
    let expanded = quote! {
        pub trait #trait_name {
            #(#items)*
        }
    };
    
    TokenStream::from(expanded)
}

/// Parses `#[getter(name: Type)]` attributes.
fn parse_getter_attr(attr: &str) -> Option<(String, String)> {
    let attr = attr.replace(' ', ""); // Remove spaces
    let parts: Vec<&str> = attr.trim_matches(|c| c == '(' || c == ')').split(':').collect();
    if parts.len() == 2 {
        Some((parts[0].to_string(), parts[1].to_string()))
    } else {
        None
    }
}

#[proc_macro_derive(ImplementTrait)]
pub fn implement_trait(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let fields = if let syn::Data::Struct(data) = &input.data {
        &data.fields
    } else {
        return syn::Error::new_spanned(input, "Only structs are supported")
            .to_compile_error()
            .into();
    };

    let getters = fields.iter().filter_map(|field| {
        let field_name = field.ident.as_ref()?;
        let field_type = &field.ty;

        Some(quote! {
            fn #field_name(&self) -> &#field_type {
                &self.#field_name
            }
        })
    });

    let expanded = quote! {
        impl AutoGetters for #struct_name {
            #(#getters)*
        }
    };

    TokenStream::from(expanded)
}
