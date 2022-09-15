// oh boy! this took me too long to finish i swear!

// lessons learned -
//      Talking about a single object:
//          - There can be only be one Owner / writer - Ownership
//          - There can be multiple refferences / readers - Borrowing
//          - There cannot be multiple Owners / writers / mutable refferences
//
//      std::cell::Cell provides mutability on immutable fields

use rand::Rng;
use std::{
    cell::Cell,
    io::{self, Write},
};

fn random_num(low: u32, high: u32) -> u32 {
    rand::thread_rng().gen_range(low..=high)
}

struct Wrestler {
    name: String,
    id: u8,
    wins: Cell<u8>,
    skills: u32,
    attack: u32,
    defence: u32,
    strength: u32,
    power: f32,
}

impl Wrestler {
    fn new(name: String, id: u8) -> Wrestler {
        Wrestler {
            name,
            id,
            wins: Cell::new(0),
            skills: random_num(75, 99),
            attack: random_num(75, 99),
            defence: random_num(75, 99),
            strength: random_num(75, 99),
            power: 0.0,
        }
    }

    fn estimate_power(&mut self) {
        self.power = (self.skills + self.attack + self.defence + self.strength) as f32 / 4.0;
    }

    fn is_stronger(&self, other: &Wrestler) -> bool {
        self.power > other.power
    }
}

fn register_wrestlers(wrestlers_list: &mut Vec<Wrestler>) {
    const NAMES: [&str; 8] = [
        "Dave", "Josh", "Ray", "Faze", "Seth", "Mac", "Billy", "Garry",
    ];

    for (i, name) in NAMES.iter().enumerate() {
        let mut wrestler = Wrestler::new(name.to_string(), i as u8);
        wrestler.estimate_power();
        wrestlers_list.push(wrestler);
    }
}

fn intro_of_participants(wrestlers_list: &Vec<Wrestler>) {
    println!("\n----------- Wrestlers Participating ----------");
    println!("----------------------------------------------\n");

    println!("NAME\tSKILLS ATTACK DEFENCE STRENGTH");
    println!("--------------------------------------");

    for wrestler in wrestlers_list {
        println!(
            "{}\t{}\t{}\t{}\t{}",
            wrestler.name, wrestler.skills, wrestler.attack, wrestler.defence, wrestler.strength
        )
    }
}

fn points_table(wrestlers_list: &Vec<Wrestler>) {
    println!("\n\n---- Points Table ----");
    println!("------------------------\n");

    println!("NAME\tWINS");
    println!("-------------");

    for wrestler in wrestlers_list {
        println!("{}\t{}", wrestler.name, wrestler.wins.get())
    }
}

fn update_ranking(wrestlers_list: &mut Vec<Wrestler>) {
    wrestlers_list.sort_by(|a, b| b.wins.cmp(&a.wins));
}

fn fight<'a>(w1: &'a Wrestler, w2: &'a Wrestler) -> &'a Wrestler {
    if w1.is_stronger(w2) {
        w1.wins.set(w1.wins.get() + 1);
        return w1;
    } else if w2.is_stronger(w1) {
        w2.wins.set(w2.wins.get() + 1);
        return w2;
    }

    // if both are equally strong
    let random = random_num(0, 1);
    match random {
        0 => {
            w1.wins.set(w1.wins.get() + 1);
            return w1;
        }
        _ => {
            w2.wins.set(w2.wins.get() + 1);
            return w2;
        }
    }
}

fn league_stage(wrestlers_list: &Vec<Wrestler>) {
    println!("\n\n ----- LEAGUE STAGE MATCHES : ----");
    println!(" -----------------------------------\n");

    for w1 in wrestlers_list {
        for w2 in wrestlers_list {
            if w1.id != w2.id {
                let winner = fight(w1, w2);
                println!(
                    "{} 🆚 {} 🥊 The Winner is 💪 {}\n",
                    w1.name, w2.name, winner.name
                );
            }
        }
    }
}

fn semi_finals(wrestlers_list: &Vec<Wrestler>) -> (&Wrestler, &Wrestler) {
    let finalist1 = fight(&wrestlers_list[0], &wrestlers_list[1]);
    let finalist2 = fight(&wrestlers_list[2], &wrestlers_list[3]);

    println!("\n\n ----- SEMIFINAL MATCHES ----");
    println!(" ---------------------------------\n");

    println!(
        "{} 🆚 {} 🥊 The Winner is 💪 {}\n",
        wrestlers_list[0].name, wrestlers_list[1].name, finalist1.name
    );

    println!(
        "{} 🆚 {} 🥊 The Winner is 💪 {}\n",
        wrestlers_list[2].name, wrestlers_list[3].name, finalist2.name
    );

    (finalist1, finalist2)
}

fn finals(finalist1: &Wrestler, finalist2: &Wrestler) {
    let winner = fight(finalist1, finalist2);

    println!("\n\n ----- THE FINALE ----");
    println!(" --------------------------------\n");

    println!(
        "{} 🆚 {} 🥊 The Winner is..\n",
        finalist1.name, finalist2.name
    );

    for _i in 0..34 {
        print!("#");
        io::stdout().flush().expect("stdout pooped!");
        std::thread::sleep(std::time::Duration::from_millis(25));
    }

    println!(
        "\n\n👑{}👑 He knocked out every opponent he faced, \nhe deserves this Wrestling Cup 🏆🏆🏆!\n\n",
        winner.name
    );
}

fn main() {
    let mut wrestlers_list: Vec<Wrestler> = Vec::new();

    register_wrestlers(&mut wrestlers_list);
    intro_of_participants(&wrestlers_list);

    league_stage(&wrestlers_list);
    update_ranking(&mut wrestlers_list);
    points_table(&wrestlers_list);

    let (finalist1, finalist2) = semi_finals(&wrestlers_list);
    finals(finalist1, finalist2);
}
