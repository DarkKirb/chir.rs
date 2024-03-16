use std::collections::HashSet;

use base64::Engine;
use proc_macro::TokenStream;
use syn::{parse::Parse, LitStr, Token};

struct StaticEmbed {
    file_name: LitStr,
    mime_type: LitStr,
}

#[derive(Debug, Clone)]
struct ExpandedStaticEmbed {
    file_name: String,
    mime_type: String,
    file_hash: String,
}

impl Parse for StaticEmbed {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let file_name: LitStr = input.parse()?;
        input.parse::<Token![=]>()?;
        let mime_type: LitStr = input.parse()?;
        Ok(StaticEmbed {
            file_name,
            mime_type,
        })
    }
}

impl StaticEmbed {
    fn expand(&self) -> std::io::Result<ExpandedStaticEmbed> {
        let file_name = self.file_name.value();
        let mime_type = self.mime_type.value();
        let content = std::fs::read(file_name.clone())?;
        let file_hash = blake3::hash(&content);
        let file_hash =
            base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(file_hash.as_bytes());
        Ok(ExpandedStaticEmbed {
            file_name,
            mime_type,
            file_hash,
        })
    }
}

impl ExpandedStaticEmbed {
    fn get_static_name(&self) -> String {
        self.file_name
            .to_uppercase()
            .replace("/", "_")
            .replace(".", "_")
            .replace("-", "_")
    }
}

struct StaticEmbeds {
    items: Vec<ExpandedStaticEmbed>,
}

impl Parse for StaticEmbeds {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut items = Vec::new();
        while !input.is_empty() {
            items.push(input.parse::<StaticEmbed>()?.expand().unwrap());
            input.parse::<Token![;]>()?;
        }
        Ok(StaticEmbeds { items })
    }
}

#[proc_macro]
pub fn static_embeds(input: TokenStream) -> TokenStream {
    let StaticEmbeds { items } = syn::parse_macro_input!(input as StaticEmbeds);
    let mut phf_map = phf_codegen::Map::new();
    let mut known_entries: HashSet<String> = HashSet::with_capacity(items.len());
    let statics = items
        .iter()
        .map(|v| {
            let ident = quote::format_ident!("{}", v.get_static_name());
            let mime_type = &v.mime_type;
            let file_hash = &v.file_hash;
            let file_name = format!("../../{}", v.file_name);
            if !known_entries.contains(file_hash) {
                phf_map.entry(file_hash, &v.get_static_name());
                known_entries.insert(file_hash.clone());
            }
            quote::quote! {
                pub static #ident: StaticFile<'static> = StaticFile {
                    mime_type: #mime_type,
                    content: StaticFileContent::Embedded(include_bytes!(#file_name)),
                    file_hash: #file_hash
                };
            }
        })
        .collect::<Vec<_>>();

    let phf_map = phf_map.build().to_string();
    let phf_map: proc_macro2::TokenStream = syn::parse_str(&phf_map).unwrap();

    let expanded = quote::quote! {
        #(#statics)*

        static STATIC_FILES: phf::Map<&'static str, StaticFile<'static>> = #phf_map;
    };

    TokenStream::from(expanded)
}
