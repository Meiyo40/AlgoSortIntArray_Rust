use std::fs;
use std::io::stdin;
use serde::Deserialize;
use serde_json::{Result, Value};
use rand::Rng;

#[derive(Deserialize, Debug)]
struct array_type_json {
    //Using vector for simplicity in the program and avoid array_size initalization
    data: Vec<i32>
}

impl array_type_json {
    ///Ordonned sort for i32 values
    pub fn sort_array(&mut self)
    {

        let mut temp = vec![0;self.data.len()]; //declaring temp mutable array of the array length

        for i in 0..self.data.len()
        {
            for j in 0..self.data.len()
            {
                if self.data[i] < self.data[j]
                {
                    temp[i] = self.data[i];
                    self.data[i] = self.data[j];
                    self.data[j] = temp[i];
                }
            }
        }
    }

    ///Remake the array with new random values of the given length
    pub fn create_new_random_array(&mut self, len: usize)
    {
        let mut rng = rand::thread_rng();
        self.data = vec![];
        for i in 0..len
        {
            let rand = rng.gen_range(0..1000);
            self.data.push(rand);
        }
    }
}

struct Game
{
    pub run: bool,
    pub array: array_type_json,
}

impl Game
{
    ///Handle user input
    pub fn new_game(&mut self)
    {
        println!("Vous voulez tester un autre tableau ?\n\
        Vous pouvez au choix:\n\
        1- Editier le fichier config.toml et remplacez les chiffres/tableau par celui que vous souhaitez (conservez la structure json).\n\
        2- Vous pouvez utiliser la commande interne pour créer un tableau de longueur N contenant des nombres de 0 à 1000\n\
        Choisissez(1/2):
        ");
        let mut choice: String = String::new();
        stdin().read_line(&mut choice);

        let index: i32 = choice.trim().parse::<i32>().unwrap();

        if index > 2
        {
            self.run = false;
            println!("Erreur dans votre choix.");
        }

        if index == 1
        {
            choice = "Vous avez choisi d'utiliser le fichier.".parse().unwrap();
            println!("{}", choice);

            Game::get_file_data();

            self.run = true;
        }
        if index == 2
        {
            choice = "Vous avez choisi d'utiliser la commande interne.\n\
            Veuillez entrer la taille du tableau désiré: ".parse().unwrap();
            println!("{}", choice);

            let mut choice: String = String::new();
            stdin().read_line(&mut choice);
            let len = choice.trim().parse::<i32>().unwrap();

            self.array.create_new_random_array(len as usize);

            self.run = true;
        }
    }

    ///Retrieve data from the file config.toml and return its Deserialized value as array_type_json type
    pub fn get_file_data() -> array_type_json
    {
        let content = fs::read_to_string(&"config.toml").unwrap();
        let array: array_type_json = serde_json::from_str(&content).unwrap();
        array
    }
}

fn main() {
    let mut game = Game {run: true, array: Game::get_file_data()};

    while game.run {
        println!("Votre tableau d'entier est: \n{:?}", &game.array.data);
        &game.array.sort_array();
        println!("Votre tableau d'entier ordonné est désormais: \n{:?}", &game.array.data);
        game.new_game();
    }

}
