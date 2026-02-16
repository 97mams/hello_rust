
//first ex 

// fn main() { 
//     let name:&str = "Mamisoa";
//     let age:i32 = 28;

//     println!("Bonjour {name}, tu as {age} ans");
// }
#[derive(Debug)]
struct User {
    first_name: String,
    last_name: String,
    age: u8
}

fn main () {
    let mut mams = User {
        first_name: String::from("Mamisoa"),
        last_name: String::from("Ratsimbarison"),
        age: 28 
    };

    mams.first_name = String::from("Santatra");

    println!("{:?}", mams);
}