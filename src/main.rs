use rand::Rng;

#[derive(Debug)]
struct Results {
    won_on_initial_selection: usize,
    won_on_second_selection: usize
}

impl Results {
    fn new() -> Results {
        Results {
            won_on_initial_selection: 0,
            won_on_second_selection: 0
        }
    }

    fn count_win_on_initial_selection(&mut self) {
        self.won_on_initial_selection += 1;
    }

    fn count_win_on_second_selection(&mut self) {
        self.won_on_second_selection += 1;
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
const IS_DEBUG: bool = false;
const SIM_COUNT: i32 = 100000;

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

fn make_second_selection(doors: &mut Vec<Door>) {
    for door in doors.iter_mut() {
        if !door.selected_1 && !door.opened {
            door.select_second()
        }
    }
}

fn is_initial_win(door: &Door) -> bool {
    door.is_gold && door.selected_1
}

fn is_second_attempt_win(door: &Door) -> bool {
    door.is_gold && door.selected_2
}

fn update_results(doors: &Vec<Door>, results: &mut Results) {
    let mut win_counted = false;

    for door in doors.iter() {
        if is_initial_win(&door) {
            results.count_win_on_initial_selection();
            win_counted = true;
            break
        }

        if is_second_attempt_win(&door) {
            results.count_win_on_second_selection();
            win_counted = true;
            break
        }
    }

    if !win_counted {
        panic!("you somehow lost both scenarios?");
    }
}

fn print_door_status(doors: &Vec<Door>, title: &str) {
    if IS_DEBUG {
        println!("{} ------------------------", title);
        print_doors(&doors);
    }
}

fn play_game(results: &mut Results) {
    let mut doors = build_doors();

    let selected_door_number = pick_random_door_number(&doors);

    doors[selected_door_number].select_initial();

    print_door_status(&doors, "Door Selection");

    open_doors(&mut doors);

    print_door_status(&doors, "Initial Door Opening");

    make_second_selection(&mut doors);

    print_door_status(&doors, "Second Door Selection");

    update_results(&doors, results);
}

fn main() {
    println!("Simulating {} games...", SIM_COUNT);

    let mut results = Results::new();

    for _ in 0..SIM_COUNT {
        play_game(&mut results);
    }

    println!("{:?}", results);
}
