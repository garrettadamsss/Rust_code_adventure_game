use read_input::prelude::*;
use random_number::random;
use std::fs::File;
use std::io::prelude::*;
use std::time::Duration;
use std::thread::sleep; 

//takes file path and prints file
fn print_ascii(file_path: String){
    let mut file = File::open(file_path).unwrap();
    let mut art = String::new();
    file.read_to_string(&mut art).unwrap();
    println!("{}", art);
}
fn main(){
   let mut gold = 100;                      //begin with 100 gold  
   let mut hero = Hero::init_hero();        //initialize a hero struct
   let time = Duration::from_secs(1);
   println!("You are the fiercest warrior known as Raheja the Centaur!");
   let file_path = String::from("ascii_art/centaur.txt");
   print_ascii(file_path);
   sleep(time);
   println!("You are on a quest to find and defeat the great dragon Placidusax");
   sleep(time);
   println!("First you decide to stop by the local trader");
   sleep(time);
   println!("...");
   sleep(time);
   println!("Hello Traveler!");
   sleep(time);
   println!("Please take a look at what I have to offer...\n\n");
   sleep(time);

   let mut choice = 0;
   while choice != 4 {
       println!("You currently have {} gold", gold);
       println!("(1) Greatsword(increase attack by 50) - 50 gold ");
       let file_path = String::from("ascii_art/sword.txt");
       print_ascii(file_path);

       println!("(2) Enchanted shield(increases defense by 20) -30 gold");
       let file_path = String::from("ascii_art/shield.txt");
       print_ascii(file_path);

       println!("(3) Dodge Potion(Increase dodge probability by 15%) -10 gold ");
       let file_path = String::from("ascii_art/potion.txt");
       print_ascii(file_path);
       
       println!("(4) Goodbye");

       choice = input::<u32>().get();
       
       if choice == 1 {
           gold -= 50;
           hero.attack_up(50);
       } else if choice == 2 {
           gold -= 30;
           hero.def_up(20);
       } else if choice == 3 {
           gold -= 10;
           hero.evade_up(0.15);
       } else if choice == 4 {
           choice = 4; 
           println!("Thank you traveler... I bid you good luck!");
           return; 
       } 
       if gold < 10 {
           choice = 4; 
           println!("Thank you traveler... I bid you good luck!");
           panic!("exiting");
       }
       println!("\n\n");
   }
   dungeon_entrance(&mut hero);
   
}

fn dungeon_entrance(hero: &mut Hero) {
    let time = Duration::from_secs(1);
    println!("You crawl through the dungeon's opening and see two paths ahead of you");
    sleep(time);
    println!("Do you choose the path to the left(1) or to the right(2)?");

    loop {
        let path_input = input::<u32>().get();

        if path_input == 1 {
            dungeon_path_left(hero);
            break;
        }
        else if path_input == 2 {
            //dungeon_path_right(hero);
            break;
        }
        else {
            println!("Don't be scared! Adventure awaits!");
        }
    }
}

fn dungeon_path_left(hero: &mut Hero) {
    let time = Duration::from_secs(1);
    println!("You walk through a dark and damp tunnel and can hear what sounds like a vicous beast close ahead!");
    println!("You enter a vast cavern and see the mighty dragon Placidusax feasting on the bones of past adventurers!");
    sleep(time);
    println!("Prepare for battle!\n");
    battle_sequence(hero);
}


// fn dungeon_path_right(hero: &Hero) {
    
// }

fn battle_sequence(hero: &mut Hero) {
    let time = Duration::from_secs(1);
    let mut dragon = Boss::init_boss();
    let file_path = String::from("ascii_art/dragon.txt");
    print_ascii(file_path);
    
    let mut choice:u32; 
    while hero.health > 0 || dragon.health > 0 {
    
        println!("Choose your move: \n(1)attack \n(2)block");
        let mut boost = 0; 
        //Player attack phase
        choice = input::<u32>().get();
        if choice == 1 {
            let damage_done = hero.deal_damage(dragon.defense); //damage_done won't be mut because for now it will do the same amount everytime
            dragon.health -= damage_done;
            println!("You swing your greatsword at Placidusax, dealing {} damage!", damage_done);
            if dragon.health <= 0 {
                println!("Placidusax lets out one final roar before he collapses to the ground");
                sleep(time);
                break;
            }
        } 
        else if choice == 2 {
            boost = 10; 
            hero.def_up(boost);
            println!("You raise your shield for the incoming attack, temporarily boosting your defense by 10 points giving you {} defense!", hero.defense);
        }
        
        //Dragon attack phase
        if dragon.health >= 400 && dragon.health <= 525{
            println!("Placidusax rears back its head and breathes a blast of fire towards you!");
            sleep(time);
            if hero.dodge_chance() == true {
                //if true player dodges
                println!("Your quick reflexes are no match for the dragon and you quickly dash behind him to avoid the fire!");
            }
            else {
                //if false player takes damage
                let fire_damage = dragon.fire_breath(hero.defense);
                hero.health -= fire_damage;
                println!("The fire scorches you, dealing {} damage!", fire_damage);
            }
            if hero.health < 0 {
                hero.health = 0;
                break;
            }
        }
        else if dragon.health >= 100 && dragon.health < 400 {
            println!("The dragon swipes its ferocious claws at you!");
            sleep(time);
            if hero.dodge_chance() == true {
                println!("Right before he strikes, you leap over his claws!");
            }
            else {
                let claw_damage = dragon.claw_attack(hero.defense);
                hero.health -= claw_damage;
                println!("Placidusax' claws pierce your armor, dealing {} damage!", claw_damage);
            }
            if hero.health < 0 {
                hero.health = 0;
                break;
            }
        }
        else if dragon.health < 100 {
            println!("As the dragon draws closer to death it lashes out in a violent rage grabbing you and launching you into the air!");
            sleep(time);
            let fall_damage = dragon.grab_attack(hero.defense);
            hero.health -= fall_damage;
            println!("You hurtle towards the floor, suffering {} points of damage!", fall_damage);
            if hero.health < 0 {
                hero.health = 0;
                break;
            }
        }
        else if dragon.health <= 0 {
            println!("Placidusax lets out one final roar before he collapses to the ground");
            sleep(time);
            break;
        }

        //Display boss and hero stats at end of each phase
        sleep(time);
        hero.remove_boost(boost);
        
        println!("\n{:#?} \n {:#?}", hero, dragon);
    }

    if dragon.health <= 0 {
        println!("Congratulations! You have slayed the dragon!");
    }

    if hero.health <= 0 {
        println!("Oh dear, you are dead!");
        let file_path = String::from("ascii_art/death.txt");
        print_ascii(file_path);
    }

}


//Main character struct
#[derive(Debug)]
struct Hero {
    health: i32, //Damage able to take
    attack: i32, //Damage dealt
    defense: i32, //Damage absorption
    evade: f64, //Dodge chance
}

//Functions for the Hero struct
impl Hero {
    //Initialize Hero base stats
    fn init_hero() -> Self {
        Self {health: 100, attack: 100, defense: 50, evade: 0.5}
    }
    
    //Heal character
    // fn heal(&mut self, amount: i32) -> i32 {
    //     self.health + amount
    // }
    
    //Increase characters damage output
    fn attack_up(&mut self, damage: i32) {
        self.attack += damage;
    }

    //Increase characters defense
    fn def_up(&mut self, defense: i32) {
        self.defense += defense;
    }

    //Remove defense boost
    fn remove_boost(&mut self, boost: i32) {
        self.defense -= boost;
    }

    //Increase characters dodge chance
    fn evade_up(&mut self, evade: f64) {
        self.evade += evade;
    }

    //Deal damage to character
    // fn take_damage(&mut self, loss: i32) -> i32 {
    //     self.health - loss
    // }
    
    //How much damage is dealt
    fn deal_damage(&mut self, defense: i32) -> i32 {
        self.attack - defense/2
    }

    //Chance to dodge an attack
    fn dodge_chance(&mut self) -> bool {
        let mut chance: f64 = random!();

        chance *= self.evade;
        if chance < 0.333 {
            false //Character hit
        }
        else {
            true //Character dodges attack
        }

    }
}

#[allow(dead_code)]
struct Weapon {
    damage: u32,
}

#[allow(dead_code)]
struct Item {
    
}

#[derive(Debug)]
struct Boss {
    health: i32,
    attack: i32,
    defense: i32,
}

impl Boss {
    
    fn init_boss() -> Boss {
        Boss {health: 525, attack: 50, defense: 150}
    }

    // fn def_up(&mut self, defense: i32) {
    //     self.defense += defense;
    // }

    fn fire_breath(&mut self, defense: i32) -> i32 {
        (defense - self.attack) + 25
    }

    fn claw_attack(&mut self, defense: i32) -> i32 {
        self.attack - defense/2
    }

    fn grab_attack(&mut self, defense: i32) -> i32 {
        self.attack - defense/4
    }

    // fn take_damage(&mut self, loss: i32) -> i32 {
    //     self.health - loss
    // } 
}

#[allow(dead_code)]
struct Enemy {
    health: u32,
    attack: u32,
    defense: u32,
}
#[allow(dead_code)]
impl Enemy {
    
    //Deal damage to enemy
    fn take_damage(&self, loss: u32) -> u32 {
        self.health - loss
    }
    
}
