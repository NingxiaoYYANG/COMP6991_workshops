/// When to use Struct? Enum? Trait?
/// 

enum CharacterClass {
    Warrior,
    Mage,
    Rogue,
}

struct Character {
    name: String,
    class: CharacterClass,
}

struct Enemy {
    name: String,
}

impl Character {
    fn attack(&self, target: &mut Enemy) {
        match self.class {
            CharacterClass::Warrior => println!("{} swings a sword at {}, deals 20 damage", self.name, target.name),
            CharacterClass::Mage => println!("{} casts a fireball at {}, deals 25 damage", self.name, target.name),
            CharacterClass::Rogue => println!("{} shoots an arrow at {}, deals 15 damage", self.name, target.name),
        }
    }
}

fn main() {
    let mut goblin = Enemy { name: String::from("Goblin")};

    let warrior = Character { name: String::from("Arthur"), class: CharacterClass::Warrior };
    let mage = Character { name: String::from("Merlin"), class: CharacterClass::Mage };
    let rogue = Character { name: String::from("Shadow"), class: CharacterClass::Rogue };

    warrior.attack(&mut goblin);
    mage.attack(&mut goblin);
    rogue.attack(&mut goblin);
}
