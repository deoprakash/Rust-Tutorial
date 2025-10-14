fn main(){
    // let empty_string = String::new(); //create new object with empty parameters
    // println!("length is {}", empty_string.len()); //0

    // let content_string = String::from("Hello World!");
    // println!("length is {}", content_string.len()); // 12

// ----------------------------------------------------------------
    // new()

    // let mut z = String::new();
    // z.push_str("Hello");
    // println!(" new String added: {}", z);

//  ----------------------------------------------------------------

    // to_string()

    // let mut name1 = "Hello World";
    // name1.push_str("Welcome");
    // println!(" new String added: {}", name1); //throw error as &str don't have push_str function.
    
    // let mut name1 = "Hello World".to_string();
    // name1.push_str("Welcome");
    // println!(" new String added: {}", name1);

    // Note: for performing any string operation, we first convert string literal (&str) to String object 

// ----------------------------------------------------------------

    // replace()

    // let name1 = "Hello, World!".to_string();
    // let name2 = name1.replace("Hello", "Howdy");
    // println!("{}",name1);
    // println!("{}",name2);

// ----------------------------------------------------------------

    // push(), push_str()  -> push() is used to append character, and push_str is used to add string. 

    // let mut a = "apple".to_string();
    // a.push('s');
    // println!("{}", a);

    // a.push_str(" is sweet.");
    // println!("{}", a);
    
// ----------------------------------------------------------------

    // len()

    // let a = "Hello".to_string();
    // println!("{}", a.len())

// ----------------------------------------------------------------

    // trim() - remove trailing and leading spaces, not the inline space.

    // let sentence = "     Today is raining.    \r\n";
    // println!("Before training ");
    // println!("length is {}", sentence.len());
    // println!();
    // println!("After training ");
    // println!("length is {}", sentence.trim().len());

// ----------------------------------------------------------------

    // split_whitespace()

    // let msg = "Hello, How are you doing today?".to_string();
    // let mut i = 1;

    // for token in msg.split_whitespace(){
    //     println!("token {}: {}", i, token);
    //     i += 1;
    // }

// ----------------------------------------------------------------

    // split()

    // let country = "India, USA, China, Sri Lanka";
    // let mut i = 1;

    // for token in country.split(","){
    //     println!("token {}: {}", i, token);
    //     i += 1;
    // }

    // // Store in a vector

    // println!("\n");
    // let tokens:Vec<&str> = country.split(",").collect();
    // println!("first is {}", tokens[0]);
    // println!("second is {}", tokens[1]);
    // println!("third is {}", tokens[2]);
    // println!("last is {}", tokens[3]);

// ----------------------------------------------------------------

    // chars() -> split string into characters.

    // let line = "Boys are playing in the field".to_string();
    // for word in line.chars(){
    //     println!("{}", word);
    // }

// ----------------------------------------------------------------

    //  concatenation of string with + operator

    // let f_name = "John".to_string();
    // let m_name  = " Sena".to_string();
    // let l_name = " Doe".to_string(); 

    // let full_name = f_name + &m_name + &l_name; //f_name -> String (first word compulsory to be string), m_name -> &str and l_name -> &str
    // println!("{}", full_name);


    // Format! Macro (Alternate option)

    // let s1 = "hello".to_string();
    // let s2 = "world".to_string();

    // let s3 = format!("{} {}", s1, s2);
    // println!("{}", s3);

// ----------------------------------------------------------------
    
    // Type casting

    // let number1 = 2025;
    // let num_to_str = number1.to_string();

    // println!("{}", number1 == 2025);
    // println!("{}", num_to_str == "2025");

// ----------------------------------------------------------------



}


// as_str()

// fn main(){
//     let example_string = String::from("example_string");
//     print_literal(example_string.as_str());
// }

// fn print_literal(data:&str){
//     println!("displaying string literal {}", data);
// }

