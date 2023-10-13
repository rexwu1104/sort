use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2, Ident as Ident2, Span as Span2};
use quote::quote;
use syn::ItemStruct;

#[proc_macro_attribute]
pub fn generate_set(_attrs: TokenStream, tokens: TokenStream) -> TokenStream {
    let tokens2 = TokenStream2::from(tokens.clone());
    let stru: ItemStruct = syn::parse(tokens).unwrap();
    let fields = stru.fields.clone();
    let functions = fields.iter()
        .filter(|f| f.ident.is_some())
        .map(|f| {
            let ident = f.ident.clone().unwrap();
            let ty = f.ty.clone();
            let new_ident = _concat_ident(vec![Ident2::new("set_", Span2::call_site()), ident.clone().into()]);
            let attrs = f.attrs.clone();
            if attrs.iter().any(|f| {
                match f.meta.clone() {
                    syn::Meta::Path(path) => {
                        path.segments.iter().any(|p| {
                            p.ident.to_string() == "ignore"
                        })
                    },
                    _ => false
                }
            }) {
                quote! {}
            } else {
                quote! {
                    #[tauri::command]
                    async fn #new_ident(#ident: #ty, state: tauri::State<'_, std::sync::Mutex<AppState>>) -> Result<(), String> {
                        let mut app = state.lock().unwrap();
                        app.#ident = #ident;
                        Ok(())
                    }
                }
            }
        })
        .collect::<Vec<TokenStream2>>();

    quote! {
        #tokens2

        #(#functions)*
    }.into()
}

#[proc_macro_attribute]
pub fn generate_get(_attrs: TokenStream, tokens: TokenStream) -> TokenStream {
    let tokens2 = TokenStream2::from(tokens.clone());
    let stru: ItemStruct = syn::parse(tokens).unwrap();
    let fields = stru.fields.clone();
    let functions = fields.iter()
        .filter(|f| f.ident.is_some())
        .map(|f| {
            let ident = f.ident.clone().unwrap();
            let ty = f.ty.clone();
            let new_ident = _concat_ident(vec![Ident2::new("get_", Span2::call_site()), ident.clone().into()]);
            let attrs = f.attrs.clone();
            if attrs.iter().any(|f| {
                match f.meta.clone() {
                    syn::Meta::Path(path) => {
                        path.segments.iter().any(|p| {
                            p.ident.to_string() == "ignore"
                        })
                    },
                    _ => false
                }
            }) {
                quote! {}
            } else {
                quote! {
                    #[tauri::command]
                    async fn #new_ident(state: tauri::State<'_, std::sync::Mutex<AppState>>) -> Result<#ty, String> {
                        let app = state.lock().unwrap();
                        Ok(app.#ident.clone())
                    }
                }
            }
        })
        .collect::<Vec<TokenStream2>>();

    quote! {
        #tokens2

        #(#functions)*
    }.into()
}

fn _concat_ident(idents: Vec<Ident2>) -> Ident2 {
    let mut ident_name = String::new();
    for ident in idents.iter() {
        ident_name.push_str(&*ident.to_string());
    }

    let new_ident = Ident2::new(&ident_name, Span2::call_site());
    new_ident
}