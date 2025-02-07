use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "assets/embed/"]
pub struct Asset;

pub const YO_MI_FILE: &str = "yomi.json";
