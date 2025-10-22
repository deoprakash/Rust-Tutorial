// Immutable

/* 
fn main(){
    let a  = 10; 
    a = 45;
}
*/

//  to make it mutable use 'mut' keyword


fn main(){
    let mut a = 10; 
    a = 45;

    println!("{}", a);
}


// Constant conventions -> const keyword with variable in uppercase

/* 
fn main(){
    const USER_NAME = "DEO";
    println!("{}", USER_NAME);
}
*/

//  shadowing variable and constant
// only allowed to variables, not constant

// fn main(){
//     let a  = 10; 
//     let a = 45; // output is 45 as 10 is overwritten

//     const NAME:&str="Deo";
//     const NAME:usize= NAME.len(); //Error : `NAME` already defined
//     println!("name changed to integer : {}",NAME);

// }


