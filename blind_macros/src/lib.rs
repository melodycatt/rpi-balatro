use proc_macro2::{TokenStream, TokenTree};
use quote::quote;
use syn::{parse_macro_input, ImplRestriction, Item, ItemStruct, ItemTrait};

#[proc_macro_attribute]
pub fn generate_trait_getters(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let attr = TokenStream::from(attr);
    let trait_def = parse_macro_input!(item as ItemTrait);
    //println!("{attr}");
    //println!("{attr:?}");

    let mut idents = attr.clone().into_iter().filter_map(|x| {
        match x {
            TokenTree::Ident(_) => Some(x),
            _ => None
        }
    });

    let mut getters = TokenStream::new();

    while let Some(first) = idents.next() {
        if let Some(second) = idents.next() {
            getters.extend(quote! { 
                fn #first(&self) -> #second;
            });
        } else {
            // If there's an odd token left, just add it alone
            panic!("no type for field name!");
        }
    }

    let trait_vis = &trait_def.vis;
    let trait_ident = &trait_def.ident;
    let trait_generics = &trait_def.generics;
    let trait_items = &trait_def.items;
    let trait_restriction = &trait_def.supertraits;

    let output = quote! {
        #trait_vis trait #trait_ident #trait_generics: #trait_restriction {
            #(#trait_items)*
            #getters
        }
    }.into();
    println!("{output}");
    output
}

#[proc_macro_attribute]
pub fn derive_trait_getters(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let attr = TokenStream::from(attr);
    let struct_def = parse_macro_input!(item as ItemStruct);
    //println!("{attr}");
    //println!("{attr:?}");

    let mut idents = attr.clone().into_iter().filter_map(|x| {
        match x {
            TokenTree::Ident(_) => Some(x),
            _ => None
        }
    });

    let trait_ident = idents.next().expect("no trait identifier!");

    let mut getters = TokenStream::new();
    let mut fields = TokenStream::new();
    while let Some(first) = idents.next() {
        if let Some(second) = idents.next() {
            getters.extend(quote! { 
                fn #first(&self) -> #second { self.#first }
            });
            fields.extend(quote! {
                #first: #second,
            });
        } else {
            // If there's an odd token left, just add it alone
            panic!("missing a token! maybe you forgot the trait identifier or the type of a field.");
        }
    }

    let struct_vis = &struct_def.vis;
    let struct_ident = &struct_def.ident;
    let struct_generics = &struct_def.generics;
    let struct_fields_punctuated = match struct_def.fields {
        syn::Fields::Named(fields) => fields,
        _ => panic!("can only be used on structs with named fields")
    }.named;
    let struct_fields = struct_fields_punctuated.iter();

    let output = quote! {
        #struct_vis struct #struct_ident #struct_generics {
            #(#struct_fields)*
            #fields
        }
        impl #struct_generics #trait_ident for #struct_ident #struct_generics {
            #getters
        }
    }.into();
    //println!("{output}");
    output
}

#[proc_macro_derive(Blind)]
pub fn derive_blind(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let struct_def = parse_macro_input!(item as ItemStruct);
    
    let struct_ident = struct_def.ident;
    let struct_generics = struct_def.generics;

    let output = quote! {
        impl #struct_generics Blind for #struct_ident #struct_generics {
            fn score_requirement(ante_score: f64) -> f64 {
                return Self::MULTIPLIER * ante_score;
            }
        } 
    };

    output.into()
}