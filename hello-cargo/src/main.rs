fn main() {
    println!("Hello, world! {}", "a todos");

    // MUTABLE VARIABLES

    let mut a_number; // convert in a mutable variable (inmutable as default)
    let b_number = "ten";

    a_number = 10;

    println!("{} {}", a_number, b_number);

    //SHADOWING
    let a = 5; // this variable exist but it can't be used 
    let a = a +5; // this binding the prev a variable with a 5 increment
    let a = a * 2; // this is the used variable 

    println!("Binding {}", a);

    // DATA TYPES
    let number: u32 = 14;
    println!("Numero: {}", number);

    let string1: &str = "jola"; 
    // &str -> string lenght known in compilation time (defined in code), string -> the length of string doesn't to be known (user input)
    // &str -> inmutable reference, string -> text data can change as your program runs

    // Specify the data type "char"
    let character_1: char = 'S';
    let character_2: char = 'f';
    
    // Complier interprets a single item in quotations as the "char" data type
    let smiley_face = '😃';

    // Complier interprets a series of items in quotations as a "str" data type and creates a "&str" reference
    let string_1 = "miley ";

    // Specify the data type "str" with the reference syntax "&str"
    let string_2: &str = "ace";

    println!("{} is a {}{}{}{}.", smiley_face, character_1, string_1, character_2, string_2);

    //TUPLES - Grouping of values of different types into one compound value

    // Declare a tuple of three elements
    let tuple_e = ('E', 5i32, true);

    // Use tuple indexing and show the values of the elements in the tuple
    println!("Is '{}' the {}th letter of the alphabet? {}", tuple_e.0, tuple_e.1, tuple_e.2);

    // if/else conditional 

    if 1 == 2 {
        println!("yes");
    } else {
        println!("No");
    }

    let formal = true;
    let greeting = if formal {
        println!("formal");
    } else {
        println!("Not formal");
    };
 
    // else if
    let num = 500; // "num" can be set at some point in the program, for now set it to 500
    let out_of_range: bool;
    if num < 0 {
        out_of_range = true;
    } else if num == 0 {
        out_of_range = true;
    } else if num > 512 {
        out_of_range = true;
    } else {
        out_of_range = false;
    }
} // end main function


