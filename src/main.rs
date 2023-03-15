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

/*  SCALAR DATA TYPE = finite set of possibilities
each of them can be compared to any other value
as less, greater, equal 

i32 asigned interger 32bit
u32 unasigned interger 32bit
*/

/*fn main() {  
    // u32 || i32 are default values 
    let x:i32 = 2;
    let x:u32 = 972;

    // u8 0 - 2^8 -1 = 0 - 255 range of numbers
    // i8 -2^7 - 2^7 - 1 = -127 - 127 range of numbers

    /* 
    i8
    i16
    i32
    i64
    i128
     */
}
*/

/*fn main() {
    // bool => 1 = true, 0 = false
    let true_or_false: bool = false;
    
    // char you must use single quote
    let letter: char = 'a';
    let number: char= '1';
}
*/



/*
    COMPOUND DATA TYPE =  tuple, array.
*/
fn main() {
    // tuple
    let tup:(i32, bool, char) = (1, true, 's');

    // can be mutable "mut"
    let mut tup2:(i16, bool, char) = (-4, false, 'd');

    // number after "." stands for index same as [] in js
    println!("{}", tup2.2); // d

    // can be changed only if mutable
    let mut tup3:(i32, bool, char) = (-11, true, 'p');
    tup3.1 = false;
    println!("{}", tup3.1);
}


/* 
Arrays -> compound
*/
fn array() {

    // can be changed if "mut"
    let arr = [1,2,3,4,5];
    arr[1]; // 2

    let arr2:[i32;3]= [1 , 3 , 4];
    arr2[2]; // 4
    println!("{}", arr2[2]);
}
