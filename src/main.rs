use std::{collections::HashMap, io};

fn main() {
    // let action = std::env::args().nth(1).expect("Please specify an action");
    // let item = std::env::args().nth(2).expect("Please specify an item");

    // // println!("{:?}, {:?}", action, item);

    // let mut todo = Todo {
    //     map: HashMap::new(),
    // };

    // if action == "add" {
    //     todo.insert(item);
    //     match todo.save() {
    //         Ok(_) => println!("saved ok"),
    //         Err(why) => println!("An error was occurred,{}", why),
    //     }
    // }

    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}

struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    fn insert(&mut self, key: String) {
        self.map.insert(key, true);
    }
    fn save(self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        for (key, val) in self.map {
            let record = format!("{}\t{}\n", key, val);
            content.push_str(&record);
        }
        std::fs::write("./db.txt", content)
    }
}
