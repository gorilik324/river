use serde::Deserialize;

#[derive(Deserialize)]
pub struct Trade {
    t: String,
    x: String,
    p: f32,
    i: u32,
    z: String,
}
