use hydroflow::hydroflow_syntax;
use multiplatform_test::multiplatform_test;

#[multiplatform_test]
pub fn test_demux_shapes() {
    enum Shape {
        Circle(f64),
        Rectangle { width: f64, height: f64 },
        Square(f64),
    }

    let mut df = hydroflow_syntax! {
        my_demux = source_iter([
            Shape::Circle(5.0),
            Shape::Rectangle { width: 10.0, height: 8.0 },
            Shape::Square(9.0),
        ]) -> demux(|shape, var_args!(circ, rect)| {
            match shape {
                Shape::Circle(radius) => circ.give(radius),
                Shape::Rectangle { width, height } => rect.give((width, height)),
                Shape::Square(side) => rect.give((side, side)),
            }
        });

        my_demux[circ] -> map(|r| std::f64::consts::PI * r * r) -> out;
        my_demux[rect] -> map(|(w, h)| w * h) -> out;
        out = union() -> for_each(|a| println!("area: {}", a));
    };
    df.run_available();
}

#[multiplatform_test]
pub fn test_demux_fizzbuzz_1() {
    let mut df = hydroflow_syntax! {
        my_demux = source_iter(1..=100)
            -> demux(|v, var_args!(fzbz, fizz, buzz, vals)|
                match v {
                    v if 0 == v % 15 => fzbz.give(()),
                    v if 0 == v % 3 => fizz.give(()),
                    v if 0 == v % 5 => buzz.give(()),
                    v => vals.give(v),
                }
            );
        my_demux[fzbz] -> for_each(|_| println!("fizzbuzz"));
        my_demux[fizz] -> for_each(|_| println!("fizz"));
        my_demux[buzz] -> for_each(|_| println!("buzz"));
        my_demux[vals] -> for_each(|x| println!("{}", x));
    };
    df.run_available();
}

#[multiplatform_test]
pub fn test_demux_fizzbuzz_2() {
    let mut df = hydroflow_syntax! {
        my_demux = source_iter(1..=100)
        -> demux(|v, var_args!(fzbz, fizz, buzz, vals)|
            match (v % 3, v % 5) {
                (0, 0) => fzbz.give(()),
                (0, _) => fizz.give(()),
                (_, 0) => buzz.give(()),
                (_, _) => vals.give(v),
            }
        );
        my_demux[fzbz] -> for_each(|_| println!("fizzbuzz"));
        my_demux[fizz] -> for_each(|_| println!("fizz"));
        my_demux[buzz] -> for_each(|_| println!("buzz"));
        my_demux[vals] -> for_each(|x| println!("{}", x));
    };
    df.run_available();
}


#[multiplatform_test]
pub fn test_switch_shapes() {
    enum Shape {
        Circle(f64),
        Rectangle { width: f64, height: f64 },
        Square(f64),
    }

    let mut df = hydroflow_syntax! {
        my_switch = source_iter([
            Shape::Circle(5.0),
            Shape::Rectangle { width: 10.0, height: 8.0 },
            Shape::Square(9.0),
        ]) -> switch(|shape: &Shape, var_args!(circ, rect)| {
            match shape {
                Shape::Circle(radius) => circ,
                Shape::Rectangle { width, height } => rect,
                Shape::Square(side) => rect,
            }
        });


        my_switch[circ] -> for_each(|a| println!("circle: {:?}", a));
        my_switch[rect] -> for_each(|a| println!("rectangle: {:?}", a));
    };
    df.run_available();
}
