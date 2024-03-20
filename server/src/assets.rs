use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "web_dist/"]
pub struct GeneratedAssets;

#[derive(RustEmbed)]
#[folder = "./"]
#[include = "*.html"]
pub struct StaticAssets;
