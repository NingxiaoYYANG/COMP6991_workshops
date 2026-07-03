/// A teaching example for enum, generics, and dynamic dispatch in Rust.
///
/// 1. Enum version: easy to start, but adding a new animal means updating match arms.
/// 2. Trait + struct version: each animal type implements the same behaviour.
/// 3. Generic function: 
/// 4. Dynamic dispatch: 

// ---------------------
// 1. Enum example
// ---------------------
enum Animal {
    Dog,
    Cat,
    Rabbit,
}

impl Animal {
    fn speak(& self) -> &str {
        match self {
            Animal::Dog => "Woof!",        
            Animal::Cat => "Meow!",
            Animal::Rabbit => "Squeak!",
        }
    }
}

// ---------------------
// 2. Trait + struct example
// ---------------------
trait AnimalBehaviour {
    fn name(&self) -> &'static str;
    fn speak(&self) -> &'static str;
}

struct Dog;
struct Cat;
struct Rabbit;

impl AnimalBehaviour for Dog {
    fn name(&self) -> &'static str {
        "dog"
    }

    fn speak(&self) -> &'static str {
        "Woof!"
    }
}

impl AnimalBehaviour for Cat {
    fn name(&self) -> &'static str {
        "cat"
    }

    fn speak(&self) -> &'static str {
        "Meow!"
    }
}

impl AnimalBehaviour for Rabbit {
    fn name(&self) -> &'static str {
        "rabbit"
    }

    fn speak(&self) -> &'static str {
        "Squeak!"
    }
}

struct snake;

// ---------------------
// 3. Generic function example
// ---------------------

// fn describe_dog(dog: &Dog) {
//     println!("{} says {}", dog.name(), dog.speak());
// }

// fn describe_cat(cat: &Cat) {
//     println!("{} says {}", cat.name(), cat.speak());
// }

// fn describe_rabbit(rabbit: &Rabbit) {
//     println!("{} says {}", rabbit.name(), rabbit.speak());
// }

// This function is not part of the trait itself.
// It is a generic helper that can work with any type implementing AnimalBehaviour.
fn describe_any<T: AnimalBehaviour>(animal: &T) 
{
    println!("{} says {}", animal.name(), animal.speak());
}

// fn describe_any(animal: &dyn AnimalBehaviour) 
// {
//     println!("{} says {}", animal.name(), animal.speak());
// }

// let d = Dog{};
// let c = Cat();
// describe_any(d);
// describe_any(c);

// ---------------------
// 4. Dynamic dispatch example
// ---------------------
fn run_dynamic_dispatch() {
    let animals: Vec<Box<dyn AnimalBehaviour>> = vec![
        Box::new(Dog{}),
        Box::new(Cat{}),
        Box::new(Rabbit{})
    ];

    for animal in animals {
        println!("{} says {}", animal.name(), animal.speak());
    }
}

// ---------------------
// 5. Trait object example
// ---------------------


fn speak_with_trait_object(animal: &dyn AnimalBehaviour) {
    println!("{} says {}", animal.name(), animal.speak());
}

fn run_trait_object_example() {
    let dog = Dog;
    let cat = Cat;

    // A trait object is a "fat pointer".
    // It is not just one pointer like `&T`.
    // Instead, it stores:
    // 1. a pointer to the actual object data (e.g. Dog/Cat/Rabbit)
    // 2. a pointer to the vtable
    //
    // The vtable is a table of function pointers for the trait methods.
    // When we call `animal.name()` or `animal.speak()`, Rust uses the vtable
    // to find the correct implementation for the concrete type.
    let dog_ref: &dyn AnimalBehaviour = &dog;
    let cat_ref: &dyn AnimalBehaviour = &cat;

    speak_with_trait_object(dog_ref);
    speak_with_trait_object(cat_ref);
}

fn main() {
    println!("Enum example:");
    let dog = Animal::Dog;
    let cat = Animal::Cat;
    let rabbit = Animal::Rabbit;
    println!("{}", dog.speak());
    println!("{}", cat.speak());
    println!("{}", rabbit.speak());

    println!("\nTrait + generic example:");
    let dog = Dog;
    describe_any(&dog);

    println!("\nDynamic dispatch example:");
    run_dynamic_dispatch();

    println!("\nTrait object example:");
    run_trait_object_example();
}


/*
high tolorence compiler: 
python
Fast and easy to see result, but more likely to get unexpected error 
when run

low tolorence compiler:
c++
Rust
More checks during compile time, prevents program to make mistake and 
unexpected behaviour during run time

*/