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
    selected_1: bool,
    selected_2: bool,
    opened: bool
}

impl Door {
    fn new() -> Door {
        Door {
            is_gold: false,
            selected_1: false,
            selected_2: false,
            opened: false
        }
    }

    fn place_gold(&mut self) {
        self.is_gold = true;
    }

    fn select_initial(&mut self) {
        self.selected_1 = true;
    }

    fn select_second(&mut self) {
        self.selected_2 = true;
    }

    fn open(&mut self) {
        self.opened = true;
    }
}

impl std::fmt::Display for Door {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Gold: {} | Initially Selected: {} | Finally Selected: {} | Open: {}",  self.is_gold, self.selected_1, self.selected_2, self.opened)
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

fn pick_random_door_number(doors: &Vec<Door>) -> usize {
    let mut rng = rand::thread_rng();
    let selected_door_number = rng.gen_range(0..doors.len()); 

    selected_door_number
}

fn open_doors(doors: &mut Vec<Door>) {
    for door in doors.iter_mut() {
        if !door.selected_1 && !door.is_gold {
            door.open();
        }
    }
}


fn main() {
    println!("Hello, world!");

    let mut results = Results::new();

    let mut doors = build_doors();

    let selected_door_number = pick_random_door_number(&doors);

    doors[selected_door_number].select_initial();

    println!("{}", "Door selection ------------------------");
    print_doors(&doors);


    open_doors(&mut doors);

    println!("{}", "Door opening ------------------------");
    print_doors(&doors);


}
