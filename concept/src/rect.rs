/* -------------------------------------------------------------------------- */
/*                                     结构体                                    */
/* -------------------------------------------------------------------------- */

#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, r2: &Rect) -> bool {
        self.width > r2.width && self.height > r2.height
    }

    fn square(width: u32) -> Rect {
        Rect {
            width,
            height: width,
        }
    }
}

fn main() {
    let r1 = Rect {
        width: 30,
        height: 50,
    };

    let r2 = Rect {
        width: 10,
        height: 40,
    };

    let r3 = Rect {
        width: 35,
        height: 55,
    };

    let a1 = r1.area();

    println!("{}", a1);

    println!("{:#?}", r1);

    println!("{}", r1.can_hold(&r2));
    println!("{}", r1.can_hold(&r3));

    let s1 = Rect::square(50);
    println!("{:?}", s1);
}
