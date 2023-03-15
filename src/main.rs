/*
rustfmt [filedir] auto text format
cargo b = build
cargo t = test
cargo c = check
cargo r = run build
cargo new [dirname] = creates rust project
*/

// fn main() {
//     print!("rust!")
// }

// fn main() {
//     let mut x = 4;
//     println!("x is: {}", x);
//     x = 5;
//     println!("x is: {}", x);
// }

// word "mut" makes the variavble mutable, can be modified later

// fn main() {
//     let mut x = 4;
//     println!("x is :{}", x);
// }

// fn main() {
//     let x = 4;
//     println!("x is :{}", x);
//     let x = x + 5;
//     println!("x is :{}", x);
// }

/*  you can access variable from exterior scope
but cannot from interior to exterior */

/*fn main() {
//     let x = 4;
//     println!("x is :{}", x);

//     {
//         let x = x - 2;
//         println!("x is :{}", x);
//     }

//     let x = x + 1;
//     println!("x is :{}", x);
// }
*/

/*
types can be changed if you define variable again
*/

/* fn main() {
    let x = 4;
    println!("x is: {}", x);

    let x = "hello";
    println!("x is:{}", x);
}*/

/*
types can not be changed if you use mutable variable
but if u define it once again it can
be done without any errors
 */

/*fn main() {
    let mut x = 4;
    println!("x is: {}", x);

    /*x = "hello";
    println!("x is :{}", x);*/

    let x = "hello";
    println!("x is :{}", x)
}
*/


/* 
const value and type can not change in the entire program
u32 = unasignes INTEGER
*/

 /* fn main() {
    const SECONDS_IN_MINUTE: u32 = 60;
    // SECONDS_IN_MINUTE = 100; error can not redefine
    println!("{}", SECONDS_IN_MINUTE);
}
*/