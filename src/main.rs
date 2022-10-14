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
    announcement("Wrestlers Participating");

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
    announcement("Points Table");

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

fn print_match_result(w1: &Wrestler, w2: &Wrestler, winner: &Wrestler) {
    println!(
        "{} 🆚 {} 🥊 The Winner is 💪 {}\n",
        w1.name, w2.name, winner.name
    );
}

fn announcement(msg: &str) {
    println!("\n\n**** {} : ****", msg);
    println!("------------------------------------------------\n");
}

fn league_stage(wrestlers_list: &Vec<Wrestler>) {
    announcement("LEAGUE STAGE MATCHES");

    for w1 in wrestlers_list {
        for w2 in wrestlers_list {
            if w1.id != w2.id {
                print_match_result(&w1, &w2, fight(w1, w2));
            }
        }
    }
}

fn semi_finals(wrestlers_list: &Vec<Wrestler>) -> (&Wrestler, &Wrestler) {
    let finalist1 = fight(&wrestlers_list[0], &wrestlers_list[1]);
    let finalist2 = fight(&wrestlers_list[2], &wrestlers_list[3]);

    announcement("SEMIFINAL MATCHES");

    print_match_result(&wrestlers_list[0], &wrestlers_list[1], &finalist1);
    print_match_result(&wrestlers_list[2], &wrestlers_list[3], &finalist2);

    (finalist1, finalist2)
}

fn finals(finalist1: &Wrestler, finalist2: &Wrestler) {
    let winner = fight(finalist1, finalist2);

    announcement("THE FINALE");

    println!(
        "{} 🆚 {} 🥊 The Winner is..\n",
        finalist1.name, finalist2.name
    );

    for _i in 0..25 {
        print!(". ");
        io::stdout().flush().expect("stdout pooped!");
        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    println!(
        "\n\n👑 {} 👑 He knocked out every opponent he faced, \nhe deserves this Wrestling Cup 🏆🏆🏆!\n\n",
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
