use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct Bar {
    t: String,
    o: f32,
    h: f32,
    l: f32,
    c: f32,
    v: i32,
    n: i32,
    vw: f32
}


