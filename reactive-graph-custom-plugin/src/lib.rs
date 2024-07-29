// We'll import the trait and the export macro from our plugin_api crate
use reactive_graph_plugin_api::bindings::export;
use reactive_graph_plugin_api::Plugin;

// Define a new struct for your plugin. Name doesn't matter
struct CustomPlugin;

// impl the trait provided by the Plugin API. rust-analyzer should tell you that it expects a function that matches the shape of the function defined in the wit file
impl Plugin for CustomPlugin {
    fn greeting(name: String) -> String {
        println!("STDIO WORKS!");
        format!("Greetings {name}! I'm a WASI plugin!")
    }
}
// Here we call the export! macro with the struct for our plugin, but since the bindings for wit are defined in plugin_api, we need to add `with_types_in` as the second arg and the path to the bindings as the third
export!(CustomPlugin with_types_in reactive_graph_plugin_api::bindings);
