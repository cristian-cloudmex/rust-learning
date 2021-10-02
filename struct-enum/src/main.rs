fn main() {
    // classic structs with named fields(like in C)
    struct Student {
        name: String,
        level: u8,
        remote: bool,
    }   

    // tupple structs with data types only
    struct Grades (
        char, 
        char,
        char,
        f32,
    );

    // Unit structs with no fields
    struct Unit;

    // Instantiate classic struct, specify fields in random order, or in specified order
    let student_1 = Student {
        name: "Cristian Zambrano".to_string(),
        level: 1,
        remote: false,
    };

    // Instantiate tuple structs, pass values in same order as types defined
    let mark_1 = Grades ('A', 'B', 'A', 3.75);

    println!("{}, level: {}, remote: {}. Grades: {}, {}, {}. Average: {}." ,
    student_1.name, student_1.level, student_1.remote, mark_1.0, mark_1.1, mark_1.2, mark_1.3);
    

    // Enum with variants

    // Set the Debug flag so we can check the data in the output
    #[derive(Debug)]
    struct MouseClick {
        x: i64, 
        y: i64
    } 

    // Set the Debug flag so we can check the data in the output
    #[derive(Debug)]
    struct KeyPress (String, char);

    // Set the Debug flag so we can check the data in the output
    #[derive(Debug)]
    enum WebEvent {
        WELoad(bool), // basic variant  
        WEKeys(KeyPress), // variant with a tuple struct
        WEClick(MouseClick), // variant with a struct
    }

    // Instantiate enum
    let we_load = WebEvent::WELoad(true);
    
    let keypress = KeyPress (String::from("CTRL +"), 'C');
    let we_keys = WebEvent::WEKeys(keypress);

    let click = MouseClick {x: 100, y: 200};
    let we_click = WebEvent::WEClick(click);

    // Print the values in the WebEvent enum variants
    // Use the {:#?} syntax to display the enum structure and data in a readable form
    println!("{:#?}, {:#?}, {:#?}", we_load, we_keys, we_click);

} // end of main
