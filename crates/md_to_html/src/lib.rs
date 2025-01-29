// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

use wasm_bindgen::prelude::*;

use comrak::{markdown_to_html, Options};

#[wasm_bindgen]
pub fn md_to_html(content: &str) -> String {
    let html : String = markdown::to_html(content);
    
    // let html2 : String = markdown_to_html(content, &Options::default());

    let mut options = Options::default();
    options.render.unsafe_ = true;

    let html2 : String = markdown_to_html(content, &options);

    format!("{}", html2)
}
    