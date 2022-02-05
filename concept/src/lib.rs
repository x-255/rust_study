mod mm {
    pub fn ff() {}

    pub struct ss {
        pub aa: u32,
        pub bb: bool,
    }
}

fn main() {
    mm::ff();

    crate::mm::ss { aa: 32, bb: true };
}
