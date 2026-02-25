
//first ex 

// fn main() { 
//     let name:&str = "Mamisoa";
//     let age:i32 = 28;

//     println!("Bonjour {name}, tu as {age} ans");
// }
// #[derive(Debug)]
// struct User {
//     first_name: String,
//     last_name: String,
//     age: u8
// }

// fn main () {
//     let mut mams = User {
//         first_name: String::from("Mamisoa"),
//         last_name: String::from("Ratsimbarison"),
//         age: 28 
//     };

//     mams.first_name = String::from("Santatra");

//     println!("{:?}", mams);
// }

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32
// }

// impl Rectangle {
//     fn square(with:u32) -> Rectangle {
//         Rectangle { width: with, height: with }
//     }

//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// fn main(){
//     let square = Rectangle::square(10);
//     println!("{:?}", square);
//     println!("L'aire du carré est de {}", square.area());
// }

// enum PokemonKind {
//     Fire,
//     Water,
//     Grass
// }

// struct Pokemon {
//     name: String,
//     kind: PokemonKind
// }

// impl Pokemon {
//     fn attack(&self) {
//         match self.kind {
//             PokemonKind::Fire => println!("{} utilise une attaque de feu !", self.name),
//             PokemonKind::Water => println!("{} utilise une attaque d'eau !", self.name),
//             PokemonKind::Grass => println!("{} utilise une attaque de plante !", self.name)
//         }
//     }
// }

// fn main() {
//     let charmander = Pokemon {
//         name: String::from("Charmander"),
//         kind: PokemonKind::Fire
//     };

//     let squirtle = Pokemon {
//         name: String::from("Squirtle"),
//         kind: PokemonKind::Water
//     };

//     let bulbasaur = Pokemon {
//         name: String::from("Bulbasaur"),
//         kind: PokemonKind::Grass
//     };

//     charmander.attack();
//     squirtle.attack();
//     bulbasaur.attack();
// }
// mod exo2;

// fn main() {
//     let mut tasks: Vec<exo2::Task> = Vec::new();

//     tasks.push(exo2::Task::new(1, String::from("Faire les courses")));
//     tasks.push(exo2::Task::new(2, String::from("Nettoyer la maison")));
//     tasks.push(exo2::Task::new(3, String::from("Faire du sport")));

//     tasks[0].is_completed();

//     exo2::Task::list_tasks(&tasks);
// }
// use std::collections::HashMap;

// fn main() {
//     // let mut scores = HashMap::new();
//     // scores.insert(String::from("FTA"), 0);
//     // scores.insert(String::from("2930"), 2);
//     let equipes: Vec<String> = vec![String::from("FTA"), String::from("2930")];
//     let scores: Vec<u32> = vec![0, 2];
//     let scores: HashMap<_, _> = equipes.into_iter().zip(scores.into_iter().map(|s| s as u32)).collect();

//     print!("{:?}", scores);
//     print!("Le score de FTA est de {}", scores.get("FTA").unwrap());
//     print!("Le score de 2930 est de {}", scores.get("2930").unwrap());

// }
 
use std::io;

fn double(n: i64, multiplier: f64) -> f64 {
    let mut init_value:f64 = 5000.0;
    let mut result = init_value;
    for _ in 0..n {
        result = init_value * multiplier;
        init_value = result;

    }
    result
}

fn main() {
    println!("Entrez un nombre :");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: i64 = input.trim().parse().expect("Please enter a valid number");
    println!("Entrez un multiplicateur :");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let multiplier: f64 = input2.trim().parse().expect("Please enter a valid number");
    let result = double(n, multiplier);
    println!("Le résultat de {} multiplié par {} est {}", n, multiplier, result);
}


