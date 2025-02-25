use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn macro_test_need_temp_db(_attribut: TokenStream, element: TokenStream) -> TokenStream {
    let input = parse_macro_input!(element as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_body = &input.block;
    let fn_attrs = &input.attrs;

    let expanded = quote! {
        #[tokio::test]
        #(#fn_attrs)*
        async fn #fn_name() {
            // Step 1 : init temp db
            let db_connection_uri = get_db_connection_uri();
            let app_db_name = get_app_db_name();
            let temp_db_name = get_temp_db_name();

            let test_db = common::database::setup(&db_connection_uri, &app_db_name, &temp_db_name).await;

            // Step 2 : execute tests
            #fn_body

            // Step 3 : clean temp db
            common::database::cleanup(&db_connection_uri, &app_db_name, &temp_db_name).await;
        }
    };

    TokenStream::from(expanded)
}
