//use std::fs::File;
#[path = "mon_module/generic.rs"]
mod generic;
// extern crate rand;
// use rand::prelude::*;

// function
// fn add(x: i32, y: i32) -> i32 {
//     x + y
// }

// fn say(text: &String) {
//     println!("{}", text);
// }

// fn edit(arg: &mut i32) {
//     *arg *= 10;
// }

// fn retour() -> &u8 {
//     let a: u8 = 10;
//     &a
// }

// fn choisir(choix: Choix) {
//     match choix {
//         Choix::Pour => println!("Pour"),
//         Choix::Contre(x) => println!("Contre parceque {}", x),
//     }
// }

// object
// #[derive(Debug)]
// struct Personne {
//     nom: String,
//     age: u8,
// }

// impl Personne {
//     fn new(n: String, a: u8) -> Personne {
//         Personne { nom: n, age: a }
//     }

//     fn hello(&self) {
//         println!("Hello je suis {} et j'ai {}", self.nom, self.age);
//     }
// }

// enum Choix {
//     Pour,
//     Contre(String),
// }

// enum Option<T> {
//     Some(T),
//     None,
// }
//
// fn op(arg: u8) -> Option<u8> {
//     Option::Some(arg)
// }
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

fn main() {
    generic::hello();
    // u8 u16 u32 u64 u128
    // u = unsigned (entier positif)
    // i8 i16 i32 i64 i128
    // i = integer (entier signe)
    // usize isize
    // f32 f64
    // let ma_variable = 12_000_000;
    // let ma_variable = 0xff;
    // let ma_variable = 0b10011;
    // let ma_variable:u8 = 12;
    // let ma_variable = 12u8;

    // reassignation
    // let mut a: u16 = 23;
    // a = 48;

    // conversion compatible type
    //let a: u16 = 23u8 as u16;
    // let a: u32 = 23.7f32 as u32;

    // constant
    // let a = 89;
    // const MA_CONSTANTE: f32 = 3.14;
    // println!("{} - {}", a, MA_CONSTANTE);

    // char
    // let a: char = 'a';
    // print!("{} ", a);

    // string
    // mutable mais pas judicieux detruit la var et la recree
    // let mut a: &str = "ma chaine de caractere";
    // a = "ma nouvelle chaine de caractere";
    // mutable et fais pour etre modifie
    // let mut b: String = String::from("une autre chaine de caractere");
    // b = String::from("changement de valeur");
    // b.push_str(" plus longue");
    // print!("{}\n{}", a, b);

    // shadowing
    // let a: u16 = 256;
    // let a = a.to_string();
    // println!("a = {}", a);

    // ownership
    // let a: u32 = 89;
    // let mut b = a;
    // b = 12;
    // println!("a = {} - b = {}", a, b);

    // Error
    // let c = "hello".to_owned();
    // let d = c;
    // println!("c = {} - d = {}", c, d);

    // resolution du probleme
    // let c = "hello".to_owned();
    // let mut d = c.clone();
    // d.push_str(" world");
    // println!("{} - {}", c, d);

    // ownership et scopes
    // let a = "hello".to_owned();
    // {
    //     let b = a;
    //     println!("b = {}", b);
    // }
    // println!("{}", b); // error b n'existe pas

    // match
    // let a = 24;
    // match a {
    //     0 => println!("zero"),
    //     _ if a < 0 => println!("erreur"),
    //     1..=17 => println!("mineur"),
    //     19 | 24 | 32 => println!("test"),
    //     _ => println!("majeur"),
    // }

    // pattern binding
    // let (a, b) = (56, 12 );
    // let (a, _, b) = (56, 12, 89);
    // let (a, (_, mut b)) = (56, (12, 89));
    // println!("a = {} - b = {}", a, b);

    // match et pattern binding
    // let a = ("add", 50, 3);

    // match a {
    //     (op, x, y) if op == "div" => println!("Result : {}, Reste {}", x / y, x % y),
    //     (op, x, y) if op == "add" => println!("Result : {}", x + y),
    //     _ => println!("Error"),
    // }
    // let a = ("pow", 4, 0);

    // match a {
    //     (op, x, y) if op == "div" => println!("Result : {}, Reste {}", x / y, x % y),
    //     (op, x, y) if op == "add" => println!("Result : {}", x + y),
    //     (op, x, _) if op == "pow" => println!("Result : {}", x * x),
    //     _ => println!("Error"),
    // }

    // let a = ("div", 4, 2);

    // let (text, result) = match a {
    //     (op, x, y) if op == "div" => ("Division", x / y),
    //     (op, x, y) if op == "add" => ("Addition", x + y),
    //     (op, x, _) if op == "pow" => ("Carre", x * x),
    //     _ => ("Error", 0),
    // };

    // println!("Operation : {} - Result : {}", text, result);

    // let mut a = String::from("hello");
    // {
    //     let b = &mut a;
    //     b.push_str(" world");
    //     println!("b = {}", b);
    // }
    // let c = &a;
    // println!("a = {}", a);

    // Array
    // let mut a: [u8; 4] = [1, 2, 3, 4];
    // let b = [[1, 2], [3, 4]];
    // a[1] *= 10;

    // println!("{:?}", a);
    // println!("{:?}", b[1][0]);

    // Slices
    // let mut a = [12, 34, 56, 78, 90];
    // let b = &mut a[0..3];
    // b[0] = 1234;
    // println!("{:?}", b);

    // tuple
    // let t1 = (1u8, 23.5, (26, "hello"));
    // let t2: (u8, i32, usize, f32) = (12, 45, 89, 20.5);
    // println!("{}", (t1.2).1)/* ; */
    // Vectors
    // let vec = vec![1, 2, 3, 4, 5];
    // let mut vec = Vec::new();
    // vec.push(56u8);
    // vec.push(12);
    // let b = &vec[0..2];
    // println!("{:?}", vec);
    // vec.remove(1);
    // println!("{:?}", vec[2]);
    // println!("{:?}", vec.get(2));
    // match vec.get(23) {
    //     Some(x) => println!("{}", x),
    //     None => println!("Error"),
    // }

    // use std::collections::HashMap;
    // Hashmap
    // let mut a = HashMap::new();

    // a.insert("a", 1);
    // a.insert("b", 2);
    // a.insert("c", 3);

    // println!("{:?}", a.get("b"));

    //String
    // let a = String::from("hello world");
    // let b = &a[0..5];
    // let c = "test";
    // println!("{}", b);

    // Loop
    // let mut count = 0;
    // let mut b = 0;
    // let result = loop {
    //     count += 1;
    //     b += 10;
    //     if count >= 10 {
    //         break b;
    //     }
    // };
    // println!("{}", result);

    // for
    // let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // for (index, elem) in a.iter().enumerate() {
    //     println!("{} - {}", index, elem);
    // }
    // for a in 1..10 {
    //     println!("{}", a);
    // }

    // label
    // 'boucle: for a in 1..10 {
    //     for b in 1..10 {
    //         if b == 3 {
    //             break 'boucle;
    //         }
    //         println!("{} - {}", a, b);
    //     }
    // }

    // functions
    // let a = add(5, 6);
    // println!("{}", a);

    // let a = String::from("hello");
    // let b = say(a);
    // let c = say(b);
    // let a = String::from("hello");
    // say(&a);
    // say(&a);

    // let mut a = 5;
    // edit(&mut a);
    // println!("{}", a);

    // module
    // mon_module::hello();

    // lib extern
    // println!("{}", random::<i8>());

    // struct
    // let dude = Personne::new("Doe".to_owned(), 35);
    // println!("{:#?}", dude);
    // dude.hello();

    // enum
    // choisir(Choix::Pour);
    // choisir(Choix::Contre("LALA".to_owned()));

    // match op(85) {
    //     Option::None => println!("Aucune valeur"),
    //     Option::Some(x) => println!("Valeur : {}", x),
    // }

    // let f = File::open("test.txt").unwrap();
    // let f = File::open("test.txt").expect("Error opening file");
    // let file = match f {
    //     Ok(content) => content,
    //     Err(error) => {
    //         panic!("Error : {}", error);
    //     }
    // };
}
