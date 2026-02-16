
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
mod exo2;

fn main() {
    let mut tasks: Vec<exo2::Task> = Vec::new();

    tasks.push(exo2::Task::new(1, String::from("Faire les courses")));
    tasks.push(exo2::Task::new(2, String::from("Nettoyer la maison")));
    tasks.push(exo2::Task::new(3, String::from("Faire du sport")));

    tasks[0].is_completed();

    exo2::Task::list_tasks(&tasks);
}