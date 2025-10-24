use rocket::fs::NamedFile;
use rocket::response::content::RawHtml;
use rocket::{Route, get, routes as rocket_routes};
use std::fs;
use std::path::{Path, PathBuf};

const DEFAULT_PLUGIN: &str = "hello-world-js";

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/main-frame")]
fn main_frame() -> Option<RawHtml<String>> {
    inject_base_tag(DEFAULT_PLUGIN)
}

fn inject_base_tag(plugin_name: &str) -> Option<RawHtml<String>> {
    let path = Path::new("plugins").join(plugin_name).join("index.html");
    let html = fs::read_to_string(path).ok()?;

    // Inject base tag after <head>
    let base_tag = format!(r#"<base href="/api/plugin/{}/">"#, plugin_name);
    let modified_html = html.replace("<head>", &format!("<head>\n  {}", base_tag));

    Some(RawHtml(modified_html))
}

#[get("/plugin/<plugin_name>/<file..>", rank = 2)]
async fn serve_plugin_file(plugin_name: &str, file: PathBuf) -> Option<NamedFile> {
    let path = Path::new("plugins").join(plugin_name).join(file);
    NamedFile::open(path).await.ok()
}

#[get("/plugin/<plugin_name>", rank = 1)]
fn serve_plugin(plugin_name: &str) -> Option<RawHtml<String>> {
    inject_base_tag(plugin_name)
}

pub fn routes() -> Vec<Route> {
    rocket_routes![index, main_frame, serve_plugin, serve_plugin_file,]
}
