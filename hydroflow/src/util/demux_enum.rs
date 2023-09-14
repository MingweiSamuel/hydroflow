use hydroflow_macro::DemuxEnum;
use pusherator::demux::PusheratorList;
use pusherator::for_each::ForEach;
use pusherator::Pusherator;
use variadics::{var_args, var_expr, var_type};

pub trait DemuxEnum<Nexts>
where
    Nexts: PusheratorList,
{
    fn demux_enum(self, outputs: &mut Nexts);
}

enum MyEnum {
    Square(usize),
    Rectangle { w: usize, h: usize },
    Circle(usize),
}
impl<Square, Rectangle, Circle> DemuxEnum<var_type!(Square, Rectangle, Circle)> for MyEnum
where
    Square: Pusherator<Item = usize>,
    Rectangle: Pusherator<Item = (usize, usize)>,
    Circle: Pusherator<Item = usize>,
    var_type!(Square, Rectangle, Circle): PusheratorList,
{
    fn demux_enum(self, var_args!(sq, re, ci): &mut var_type!(Square, Rectangle, Circle)) {
        match self {
            MyEnum::Square(s) => sq.give(s),
            MyEnum::Rectangle { w, h } => re.give((w, h)),
            MyEnum::Circle(r) => ci.give(r),
        }
    }
}

#[test]
fn test() {
    let val = MyEnum::Rectangle { w: 5, h: 6 };
    let mut nexts = var_expr!(
        ForEach::new(|x| println!("1 {:?}", x)),
        ForEach::new(|x| println!("2 {:?}", x)),
        ForEach::new(|x| println!("3 {:?}", x)),
    );
    val.demux_enum(&mut nexts);
}

#[derive(DemuxEnum)]
enum MyEnum2 {
    Square(usize),
    Rectangle { w: usize, h: usize },
    Circle { r: usize },
}

#[test]
fn test2() {
    let val = MyEnum2::Rectangle { w: 5, h: 6 };
    let mut nexts = var_expr!(
        ForEach::new(|x| println!("1 {:?}", x)),
        ForEach::new(|x| println!("2 {:?}", x)),
        ForEach::new(|x| println!("3 {:?}", x)),
    );
    val.demux_enum(&mut nexts);
}
