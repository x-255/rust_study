use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let screct_num = rand::thread_rng().gen_range(1, 101);

    // println!("神秘数字是：{}", screct_num);

    loop {
        println!("猜一个数");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("无法读取行!");

        let guess: u32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        println!("你输入的数是：{}", guess);

        match guess.cmp(&screct_num) {
            Ordering::Less => println!("小了！"),
            Ordering::Greater => println!("大了！"),
            Ordering::Equal => {
                println!("对了！");
                break;
            }
        }
    }
}
