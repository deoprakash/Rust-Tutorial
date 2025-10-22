fn main(){

    //------------- FOR LOOP ------------------------------

    // for x in 1..11{
    //     if x ==5{
    //         continue;
    //     }
    //     println!("{}", x);
    // }

    // -------------------wHILE -------------------------------

    // let mut x  = 0;
    // while x <10{
    //     x +=1;

    //     if x%2 == 0{
    //         continue
    //     }
    //     println!("{}", x);
    // }
    // println!("{}", x);

    // -------------------- LOOP ------------------------


    let mut x = 0;
    loop {
        x+=1;
        println!("x = {}", x);

        if x == 15{
            break;
        }
    }

}