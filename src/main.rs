use rand::Rng;

#[derive(Debug)]
struct Results {
    won: usize,
    lost: usize,
    won_on_initial_selection: usize
}

impl Results {
    fn new() -> Results {
        Results {
            won: 0,
            lost: 0,
            won_on_initial_selection: 0
        }
    }

    fn count_win(&mut self) {
        self.won += 1;
    }

    fn count_loss(&mut self) {
        self.lost += 1;
    }

    fn count_win_on_initial_selection(&mut self) {
        self.won += 1;
        self.won_on_initial_selection += 1;
    }
}

struct Door {
    is_gold: bool,
    selected: bool,
    opened: bool
}

impl Door {
    fn new() -> Door {
        Door {
            is_gold: false,
            selected: false,
            opened: false
        }
    }

    fn place_gold(&mut self) {
        self.is_gold = true;
    }

    fn select(&mut self) {
        self.selected = true;
    }

    fn open(&mut self) {
        self.opened = true;
    }
}

impl std::fmt::Display for Door {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Gold: {} | Selected: {} | Open: {}",  self.is_gold, self.selected, self.opened)
    }
}

const DOOR_COUNT: usize = 3;

fn print_doors(doors: &Vec<Door>) {
    for i in 0..doors.len() {
        println!("{}", doors[i])
    }
}

fn build_doors() -> Vec<Door> {
    let mut doors: Vec<Door> = Vec::with_capacity(DOOR_COUNT);

    let mut rng = rand::thread_rng();
    let gold_door_number = rng.gen_range(0..DOOR_COUNT); 

    for i in 0..DOOR_COUNT {
        let mut door = Door::new();
        if i == gold_door_number {
            door.place_gold();
        }

        doors.push(door);
    }

    println!("{}", "Door init ------------------------");
    print_doors(&doors);

    doors
}

fn main() {
    println!("Hello, world!");

    let mut results = Results::new();

    let mut doors = build_doors();


    let mut rng = rand::thread_rng();
    let selected_door_number = rng.gen_range(0..DOOR_COUNT); 

    doors[selected_door_number].select();

    println!("{}", "Door selection ------------------------");
    print_doors(&doors);

    for door in doors.iter() {
        if door.is_gold && door.selected {
            results.count_win_on_initial_selection()
        }
    }


    

}
