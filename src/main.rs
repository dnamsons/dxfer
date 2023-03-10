use dxf2image::{dxf2svg};

fn main() {
    let dxf = "./sample.dxf";

    // Convert to svg
    let svg = "sample.svg";
    dxf2svg(dxf, svg).unwrap();
}
