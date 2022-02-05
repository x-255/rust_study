// #[derive(Debug)]
// enum Eee {
//     a(u32),
//     b { a: bool, b: u32 },
// }

fn main() {
    // let a = Eee::a(11);
    // let b = Eee::b { a: false, b: 24 };

    let n = Some(11);
    let s = Some("xxx");
    let b: Option<u32> = None;

    println!("{:?}", n);
}
