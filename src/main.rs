use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

#[derive(Serialize, Deserialize)]
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
			(_, hun) if hun > 80 => "(>_<) Hangry",		//check hangry first to pass the test?
            (h, _) if h > 80 => "(*_*) Ecstatic",
            (h, hun) if h > 60 && hun < 50 => "(^_^) Happy",           
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

    fn save_to_file(&self) -> io::Result<()> {
        let save_data = serde_json::to_string(&self)?;
        fs::write(format!("{}.json", self.name), save_data)?;
        println!("Game saved successfully!");
        Ok(())
    }

    fn load_from_file(name: &str) -> io::Result<Pet> {
        let save_data = fs::read_to_string(format!("{}.json", name))?;
        let pet: Pet = serde_json::from_str(&save_data)?;
        Ok(pet)
    }
	
	fn delete_save(&self) -> io::Result<()> {
		let file_path = format!("{}.json", self.name);
		if fs::metadata(&file_path).is_ok() {
		fs::remove_file(&file_path)?;
		println!("Save file for {} deleted successfully!", self.name);
		} else{
		println!("No save file exists for {}", self.name);
		}
		Ok(())
	}
}

fn display_title() {
    println!("\n==========================================");
    println!("    VIRTUAL PET GAME");
    println!("         ╭━━━━ ");
    println!("         ┃^ω^┃ ");
    println!("         ╰━━━━ ");
    println!("==========================================\n");
}

fn display_main_menu() -> String {
    println!("\nMAIN MENU");
    println!("1. New Game");
    println!("2. Load Game");
    println!("3. Delete Save");
	println!("4. Exit");
    
    print!("Choose an option (1-4): ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn game_menu(pet: &mut Pet) -> bool {
    pet.status();
    pet.random_event();
    
    println!("\nWhat would you like to do?");
    println!("1. Feed pet    (*)");
    println!("2. Play with pet (^)");
    println!("3. Let pet sleep (z)");
    println!("4. Save game");
    println!("5. Delete save");
	println!("6. Return to main menu");
    
    print!("Choose an option (1-6): ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    match input.trim() {
        "1" => pet.feed(),
        "2" => pet.play(),
        "3" => pet.sleep(),
        "4" => {
            if let Err(e) = pet.save_to_file() {
                println!("Error saving game: {}", e);
            }
        }
		"5" => {
			println!("Are you sure you want to delete this save file? (y/n)");
			let mut confirm = String::new();
			io::stdin().read_line(&mut confirm).unwrap();
			if confirm.trim().to_lowercase() == "y" {
				if let Err(e) = pet.delete_save() {
					println!("Error deleting save: {}", e);
				}
			}
		}
        "6" => return false,
        _ => println!("Invalid option! Please try again."),
    }
    
    thread::sleep(Duration::from_secs(1));
    true
}

fn create_new_pet() -> Pet {
    print!("Enter your pet's name: ");
    io::stdout().flush().unwrap();
    
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    Pet::new(name.trim().to_string())
}

fn load_game() -> Option<Pet> {
    print!("Enter the name of your pet to load: ");
    io::stdout().flush().unwrap();
    
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim();
    
    match Pet::load_from_file(name) {
        Ok(pet) => {
            println!("Game loaded successfully!");
            Some(pet)
        }
        Err(_) => {
            println!("No save file found for pet named '{}'", name);
            None
        }
    }
}

fn delete_save_file() {
	print!("Enter the name of the pet save to delete: ");
	io::stdout().flush().unwrap();
	
	let mut name = String::new();
	io::stdin().read_line(&mut name).unwrap();
	let name = name.trim();
	
	println!("Are you sure you want to delete the save file for {}? (y/n)", name);
	let mut confirm = String::new();
	io::stdin().read_line(&mut confirm).unwrap();
	
	if confirm.trim().to_lowercase() == "y" {
		let file_path = format!("{}.json", name);
		if fs::metadata(&file_path).is_ok() {
			if let Err(e) = fs::remove_file(&file_path) {
				println!("Error deleting save file: {}", e);
			} else {
				println!("Save file for {} deleted successfully!", name);
			}
		} else {
			println!("No save file exists for {}", name);
		}
	}
}

fn main() {
    display_title();
    
    loop {
        match display_main_menu().as_str() {
            "1" => {
                let mut pet = create_new_pet();
                while game_menu(&mut pet) {}
            }
            "2" => {
                if let Some(mut pet) = load_game() {
                    while game_menu(&mut pet) {}
                }
            }
			"3" => delete_save_file(),
            "4" => {
                println!("Thanks for playing! Goodbye! (^_^)/");
                break;
            }
            _ => println!("Invalid option! Please try again."),
        }
    }
}

/*------------------------------------Tests---------------------------------*/

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    //create a test pet
    fn create_test_pet() -> Pet {
        Pet::new("TestPet".to_string())
    }

    #[test]
    fn test_new_pet_initialization() {
       let pet = create_test_pet();
       assert_eq!(pet.name, "TestPet");
	   assert_eq!(pet.hunger, 50);
	   assert_eq!(pet.happiness, 50);
	   assert_eq!(pet.energy, 100);
	}

	#[test]
	fn test_feed_pet() {
		let mut pet = create_test_pet();
		pet.hunger = 80;
		pet.feed();	
		assert!(pet.hunger < 80);
		assert!(pet.hunger >= 0);
		assert!(pet.happiness <= 100);
	}

	#[test]
	fn test_play_with_pet() {
		let mut pet = create_test_pet();
		let initial_energy = pet.energy;
		let initial_happiness = pet.happiness;
		
		pet.play();
		
		assert!(pet.energy < initial_energy);
		assert!(pet.happiness > initial_happiness);
		assert!(pet.energy >= 0);
		assert!(pet.energy <= 100);
	}

	#[test]
	fn test_tired_pet_can_play() {
		let mut pet = create_test_pet();
		pet.energy = 10; 	//energy below the limit for the ability to play
		let initial_happiness = pet.happiness;
		
		pet.play();
		
		assert_eq!(pet.happiness, initial_happiness); 	//happiness value should stay the same
	}

	#[test]
	fn test_sleep_restores_energy() {
		let mut pet = create_test_pet();
		pet.energy = 30;
		pet.sleep();
		assert_eq!(pet.energy, 100);
	}

	#[test]
	fn test_stats_clamping() {
		let mut pet = create_test_pet();
		
		//test the upper bounds
		pet.happiness = 150;
		pet.feed();		//should clamp happiness
		assert_eq!(pet.happiness, 100);
		
		//test the lower bounds
		pet.hunger = -50;
		pet.play(); 	//should clamp hunger
		assert_eq!(pet.hunger, 0);
	}

	#[test]
    fn test_mood_changes() {
        let mut pet = create_test_pet();
        
        // test ecstatic mood
        pet.happiness = 90;
        pet.hunger = 30;
        assert_eq!(pet.get_mood(), "(*_*) Ecstatic");
        
        // test hangry mood
        pet.hunger = 90;
        assert_eq!(pet.get_mood(), "(>_<) Hangry");
    }


	#[test]
	fn test_save_and_load() -> io::Result<()> {
		let dir = tempdir()?;
		let file_path = dir.path().join("TestPet.json");
		
		let test_pet = Pet {
			name: "TestPet".to_string(),
			hunger: 60,
			happiness: 70,
			energy: 80,
		};
		
		//test saving function
		fs::write(&file_path, serde_json::to_string(&test_pet)?)?;
		
		//test loading save file function
		let loaded_pet: Pet = serde_json::from_str(&fs::read_to_string(&file_path)?)?;
		
		assert_eq!(test_pet.name, loaded_pet.name);
		assert_eq!(test_pet.hunger, loaded_pet.hunger);
		assert_eq!(test_pet.happiness, loaded_pet.happiness);
		assert_eq!(test_pet.energy, loaded_pet.energy);
		
		Ok(())
	}

	#[test]
	fn test_display_stat() {
		let pet = create_test_pet();
		let stat = pet.display_stat(50);
		assert!(stat.contains("█████")); 	// should have 5 full blocks
		assert!(stat.contains("░░░░░")); 	// should have 5 empty blocks
		assert!(stat.contains("(50)")); 	// should show the number
	}


}







































