use rand::Rng;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

struct Pet {
    name: String,
    hunger: i32,
    happiness: i32,
    energy: i32,
}

impl Pet {
    fn new(name: String) -> Pet {
        Pet {
            name,
            hunger: 50,
            happiness: 50,
            energy: 100,
        }
    }

    fn feed(&mut self) {
        println!("(*) Feeding {}...", self.name);
        self.hunger = (self.hunger - 30).clamp(0, 100);
        self.happiness = (self.happiness + 10).clamp(0, 100);
        println!("Yum! {} looks satisfied!", self.name);
    }

    fn play(&mut self) {
        if self.energy < 20 {
            println!("(z) {} is too tired to play!", self.name);
            return;
        }
        println!("(^) Playing with {}...", self.name);
        self.happiness = (self.happiness + 20).clamp(0, 100);
        self.energy = (self.energy - 20).clamp(0, 100);
        self.hunger = (self.hunger + 10).clamp(0, 100);
        println!("{} had fun playing!", self.name);
    }

    fn sleep(&mut self) {
        println!("(~) {} is taking a nap...", self.name);
        thread::sleep(Duration::from_secs(2));
        self.energy = 100;
        self.hunger = (self.hunger + 10).clamp(0, 100);
        println!("{} wakes up feeling refreshed!", self.name);
    }

    fn status(&self) {
        println!("\n=== {}'s Status ===", self.name);
        println!("Hunger:    {}", self.display_stat(self.hunger));
        println!("Happiness: {}", self.display_stat(self.happiness));
        println!("Energy:    {}", self.display_stat(self.energy));
        println!("Mood:      {}", self.get_mood());
    }

      fn display_stat(&self, value: i32) -> String {
        let bars = "█".repeat((value / 10) as usize);
        let spaces = "░".repeat((10 - (value / 10)) as usize);
        format!("{}{} ({})", bars, spaces, value)
    }

    fn get_mood(&self) -> &str {
        match (self.happiness, self.hunger) {
            (h, _) if h > 80 => "(*_*) Ecstatic",
            (h, hun) if h > 60 && hun < 50 => "(^_^) Happy",
            (_, hun) if hun > 80 => "(>_<) Hangry",
            (h, _) if h < 30 => "(;_;) Sad",
            _ => "(-_-) Content",
        }
    }

    fn random_event(&mut self) {
        let mut rng = rand::thread_rng();
        if rng.gen_ratio(1, 5) {
            let events = [
                "found a treat! (+10 happiness)",
                "did some exercise! (-10 energy, +5 happiness)",
                "took a quick nap! (+20 energy)",
            ];
            let event = events[rng.gen_range(0..events.len())];
            println!("\n(*) {} {}", self.name, event);
            match event {
                e if e.contains("treat") => {
                    self.happiness = (self.happiness + 10).clamp(0, 100);
                }
                e if e.contains("exercise") => {
                    self.energy = (self.energy - 10).clamp(0, 100);
                    self.happiness = (self.happiness + 5).clamp(0, 100);
                }
                e if e.contains("nap") => {
                    self.energy = (self.energy + 20).clamp(0, 100);
                }
                _ => {}
            }
        }
    }
}

fn main() {
    println!("Welcome to Virtual Pet!");
    println!("(^_^)");
    print!("Enter your pet's name: ");
    io::stdout().flush().unwrap();
    
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim().to_string();
    
    let mut pet = Pet::new(name);
    
    loop {
        pet.status();
        pet.random_event();
        
        println!("\nWhat would you like to do?");
        println!("1. Feed pet    (*)");
        println!("2. Play with pet (^)");
        println!("3. Let pet sleep (z)");
        println!("4. Exit");
        
        print!("Choose an option (1-4): ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim() {
            "1" => pet.feed(),
            "2" => pet.play(),
            "3" => pet.sleep(),
            "4" => {
                println!("Goodbye! Take care of {} (^_^)/", pet.name);
                break;
            }
            _ => println!("Invalid option! Please try again."),
        }
        
        thread::sleep(Duration::from_secs(1));
    }
}