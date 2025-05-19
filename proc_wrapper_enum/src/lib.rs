//use std::char;

//use itertools::{Itertools, IntoChunks, Chunk, Chunks, ChunkBy};
//use proc_macro::Ident;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemEnum};

#[proc_macro_attribute]
pub fn wrapper_enum(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    //let attr = TokenStream::from(attr);
    //let prefix = attr.into_iter().next().unwrap().to_string();
    let enum_def = parse_macro_input!(item as ItemEnum);
    //println!("{attr}");
    //println!("{attr:?}");
    let enum_vis = &enum_def.vis;
    let enum_ident = &enum_def.ident;
    let enum_generics = &enum_def.generics;
    let enum_variants = enum_def.variants.iter();
    let j_variants = enum_variants.clone().map(|x| {
        format_ident!("J{}", x.ident)
    });

    let match_variants1 = enum_variants.clone();
    let match_variants2 = enum_variants.clone();
    let match_variants3 = enum_variants.clone();
    let match_variants4 = enum_variants.clone();
    let match_variants5 = enum_variants.clone();
    let match_variants6 = enum_variants.clone();
    let match_variants7 = enum_variants.clone();
    let match_variants8 = enum_variants.clone();
    let method_defs = quote! {
        fn enhancements(&mut self) -> &mut JokerEnhancements {
            match self {
                #(Self::#match_variants1(__we_self) => __we_self.enhancements()),*
            }
        }
        fn name(&self) -> &'static str {
            match self {
                #(Self::#match_variants2(__we_self) => __we_self.name()),*
            }
        }
        fn id(&self) -> &'static str {
            match self {
                #(Self::#match_variants3(__we_self) => __we_self.id()),*
            }
        }
        fn buy<T: DeckType>(&self, game: &mut Game<T>) {
            match self {
                #(Self::#match_variants4(__we_self) => __we_self.buy(game)),*
            }
        }
        fn sell<T: DeckType>(&self, game: &mut Game<T>)  {
            match self {
                #(Self::#match_variants5(__we_self) => __we_self.sell(game)),*
            }
        }
        fn apply<T: DeckType>(&self, chips: &mut f64, mult: &mut f64, game: &mut Game<T>, hand: HandType)  {
            match self {
                #(Self::#match_variants6(__we_self) => __we_self.apply(chips, mult, game, hand)),*
            }
        }
        fn score<T: DeckType>(&self, chips: &mut f64, mult: &mut f64, card: &mut Card, game: &mut Game<T>, retrigger: bool) -> bool {
            match self {
                #(Self::#match_variants7(__we_self) => __we_self.score(chips, mult, card, game, retrigger)),*
            }
        }
        fn cashout(&self, round_data: &RoundData, game_data: &GameData) -> usize {
            match self {
                #(Self::#match_variants8(__we_self) => __we_self.cashout(round_data, game_data)),*
            }
        }
    };

    let wrapper_def = quote! {
        #[derive(Serialize, Deserialize)]
        #enum_vis enum #enum_ident #enum_generics {
            #(#enum_variants(#j_variants)),*
        }
    };

    let output = quote! {
        #wrapper_def
        impl<'de> JokerType<'de> for #enum_ident #enum_generics {
            #method_defs
        }
    }.into();
    //println!("{output}");
    output
}

