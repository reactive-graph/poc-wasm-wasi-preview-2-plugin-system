pub mod bindings {

    use wit_bindgen::generate;
    // The first arg is the path to our wit file, the second tells it to make our export macro public(which we'll need), and the third will let us change the macro name
    generate!({path: "./wit/reactive-graph-plugin.wit", pub_export_macro: true, export_macro_name: "export"  });
}

// Reexport the Guest trait as a different name. entirely optional
pub use crate::bindings::{export, Guest as Plugin};
