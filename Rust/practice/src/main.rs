// // fn main() {
// //     println!("Hello, world!");
// // }
// fn another_function(){
//     println!("Another Function!");
// }

// fn main(){
//     // let mut x = 5;
//     // println!("the value of x is :{x}");
//     // x =6;
//     // println!("the value of x is :{x}");
//     // const THREE_HRS_IN_SEC : u32 = 60 * 60 * 3;
//     // println!("the value of x is : {THREE_HRS_IN_SEC}");
//     // let x =5 ;
//     // let x = x+1;
//     // {
//     //     let x = x*2;
//     //     println!("the value of x in the inner scope is :{x}");

//     // }
//     // println!("the value of x is :{x}");

//     // let mut spaces = "    ";
//     // let spaces = spaces.len();
//     // println!("the value of spaces is :{spaces}");
//     // let guess: u32 = "42".parse().expect("Not a number!");
//     // println!("the value of guess is :{guess}");
//     // let c = 'z';
//     // let z: char = 'ℤ'; // with explicit type annotation
//     // let heart_eyed_cat = '😻';

//     // let tup: (i32, f64, u8) = (500, 6.4, 1);
//     // println!("the value of tup is :{tup:?}");
//     // let (x, y, z) = tup;
//     // println!("the value of x is :{x}");
//     // println!("The value of y is: {y}");
//     // println!("The value of z is : {z}");
//     println!("Hello World!");

//     another_function();



// }

fn main(){
    let y = {
        let x=3;
        x+1
    };
    println!("the value of y is :{y}");
}