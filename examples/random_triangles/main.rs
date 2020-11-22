extern crate jam;

use jam::toolbox::Triangle;

use svg::Document;
use svg::node::element::Path;


fn main() {
    let size = 10000;
    let mut buffer = vec![];
    for _ in 1..size  {
        let triangle = Triangle::random((700, 700), (10, 20))
            .rotate_random()
            .draw();

        &buffer.push(triangle);
     }

    let p = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 0.1)
        .set("d", buffer);

    let document = Document::new().set("viewBox", (0, 0, 800, 800))
        .add(p);

    svg::save("image.svg", &document).unwrap();
}
