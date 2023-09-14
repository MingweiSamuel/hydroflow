use hydroflow_macro::{hydroflow_syntax, DemuxEnum};
use multiplatform_test::multiplatform_test;

#[multiplatform_test]
pub fn test_demux_enum() {
    #[derive(DemuxEnum)]
    enum Shape {
        Square(f64),
        Rectangle { w: f64, h: f64 },
        Circle { r: f64 },
    }

    let mut df = hydroflow_syntax! {
        my_demux = source_iter([
            Shape::Rectangle { w: 10.0, h: 8.0 },
            Shape::Square(9.0),
            Shape::Circle { r: 5.0 },
        ]) -> demux_enum();

        my_demux[Circle] -> map(|(r,)| std::f64::consts::PI * r * r) -> out;
        my_demux[Rectangle] -> map(|(w, h)| w * h) -> out;
        my_demux[Square] -> map(|s| s * s) -> out;

        out = union() -> for_each(|area| println!("Area: {}", area));
    };
    df.run_available();
}
