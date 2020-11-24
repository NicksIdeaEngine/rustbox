// structs - used to create custom data types

// traditional struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// tuple struct
struct TupColor(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // person constructor
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    // get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    // traditional struct
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };

    println!("color: {} {} {}", c.red, c.green, c.blue);
    c.red = 200;
    println!("color: {} {} {}", c.red, c.green, c.blue);

    // tuple struct
    let mut t = TupColor(205, 19, 44);
    println!("tupcolor: {} {} {}", t.0, t.1, t.2);
    t.1 = 75;
    println!("tupcolor: {} {} {}", t.0, t.1, t.2);

    // construct a new person
    let p = Person::new("Nick", "Geeves");
    println!("person: {} {}", p.first_name, p.last_name);

    // get full name
    let pfull = Person::new("Steve", "Moo");
    println!("full name: {}", pfull.full_name());

    // set new last name
    let mut plast = Person::new("Trish", "Grimes");
    println!("change last name: {}", plast.full_name());
    plast.set_last_name("Eriksun");
    println!("change last name: {}", plast.full_name());

    // to tuple
    println!("to tuple: {:?}", plast.to_tuple());
}
