use std::collections::HashSet;

use base64::Engine;
use proc_macro::TokenStream;

fn get_static_name(name: &str) -> String {
    format!("ASSET_{}", name.to_ascii_uppercase().replace('.', "_"))
}

#[proc_macro]
pub fn static_embeds(_: TokenStream) -> TokenStream {
    let dir = std::fs::read_dir("./web/dist").unwrap();
    let mut statics = Vec::new();
    let mut phf_map = phf_codegen::Map::new();
    let mut known_entries: HashSet<String> = HashSet::new();
    for entry in dir {
        let entry = entry.unwrap();
        let file_name = entry.file_name().to_string_lossy().to_string();
        let mime_type = mime_guess::from_path(&file_name)
            .first_or_octet_stream()
            .to_string();
        let content = std::fs::read(format!("web/dist/{file_name}")).unwrap();
        let file_hash = blake3::hash(&content);
        let file_hash =
            base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(file_hash.as_bytes());
        if !known_entries.contains(&file_hash) {
            phf_map.entry(file_name.clone(), &get_static_name(&file_name));
            known_entries.insert(file_hash.clone());
        }
        let ident = quote::format_ident!("{}", get_static_name(&file_name));
        let file_name = format!("../../web/dist/{file_name}");
        statics.push(quote::quote! {
            #[allow(missing_docs)]
            pub static #ident: StaticFile<'static> = StaticFile {
                mime_type: #mime_type,
                content: StaticFileContent::Embedded(include_bytes!(#file_name)),
                file_hash: #file_hash
            };
        });
    }

    let phf_map = phf_map.build().to_string();
    let phf_map: proc_macro2::TokenStream = syn::parse_str(&phf_map).unwrap();

    let expanded = quote::quote! {
        #(#statics)*

        #[allow(clippy::missing_docs_in_private_items)]
        static STATIC_FILES: phf::Map<&'static str, StaticFile<'static>> = #phf_map;
    };

    TokenStream::from(expanded)
}
