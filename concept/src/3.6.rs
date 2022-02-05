/**
 * 控制流：if else、循环
 */


/* --------------------------------- if else -------------------------------- */
    // let n = 4;

    /* if n % 4 == 0 {
        println!("by 4")
    } else if n % 3 == 0 {
        println!("by 3")
    } else if n % 2 == 0 {
        println!("by 2")
    } else {
        println!("not by 4,3,2")
    } */

    /* let y = if n > 5 { true } else { false };
    println!("{}", y) */

    /* ---------------------------------- loop ---------------------------------- */
    /* let mut x = 1;
    let res = loop {
        x += 1;
        println!("x={}", x);
        if x == 10 {
            break x * 2;
        }
    };

    println!("{}", res) */

    /* ---------------------------------- while --------------------------------- */
    /* let mut x = 4;

    while x != 0 {
        println!("{}!", x);
        x = x - 1;
    }

    println!("Biu!") */

    /* ----------------------------------- for ---------------------------------- */
    /* let arr = [11, 22, 33, 44, 55];

    for val in arr {
        println!("val: {}", val);
    }

    for i in (1..4).rev() {
        println!("{}!", i);
    }
    println!("Biu!"); */