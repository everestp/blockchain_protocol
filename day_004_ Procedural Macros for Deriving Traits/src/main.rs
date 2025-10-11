use std::fmt;

// ----------------------- Serialization/Deserialization Traits and Implementation -----------------------

// Trait for serializing structs into a Vec<u8> (changed from Vec<u32> to match byte-level serialization)
trait Serialize1 {
    fn serialize(&self) -> Vec<u8>; // Changed to Vec<u8>
}

// Trait for deserializing a byte slice into a struct
trait Deserialize {
    fn deserialize(v: &[u8]) -> Result<Self, fmt::Error> where Self: Sized;
}

// Struct representing a swap with two quantities
#[derive(Debug)]
struct Swap {
    qty_1: u32, // First quantity
    qty_2: u32, // Second quantity
}

// Implementation of Serialize1 for Swap
impl Serialize1 for Swap {
    fn serialize(&self) -> Vec<u8> { // Changed return type to Vec<u8>
        let mut v = vec![];
        // Convert qty_1 to big-endian bytes and extend the vector
        v.extend_from_slice(&self.qty_1.to_be_bytes());
        // Convert qty_2 to big-endian bytes and extend the vector
        v.extend_from_slice(&self.qty_2.to_be_bytes());
        v // Return the vector of bytes
    }
}

// Implementation of Deserialize for Swap
impl Deserialize for Swap {
    fn deserialize(data: &[u8]) -> Result<Swap, fmt::Error> {
        // Check if the input slice has at least 8 bytes (4 for each u32)
        if data.len() < 8 {
            return Err(fmt::Error);
        }
        // Extract first 4 bytes and convert to u32 (big-endian)
        let qty_1 = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
        // Extract next 4 bytes and convert to u32 (big-endian)
        let qty_2 = u32::from_be_bytes([data[4], data[5], data[6], data[7]]);
        // Return a new Swap instance wrapped in Ok
        Ok(Swap { qty_1, qty_2 })
    }
}

// ----------------------- Shape Trait and Implementations -----------------------

// Trait defining methods for calculating area and perimeter of shapes
trait Shape {
    fn area(&self) -> u32;
    fn perimeter(&self) -> u32;
}

// Struct representing a rectangle with width and height
struct Rect {
    width: u32,
    height: u32,
}

// Struct representing a square with a side length
struct Square {
    side: u32,
}

// Struct representing a circle with a radius
struct Circle {
    radius: u32,
}

// Implementation of Shape for Rect
impl Shape for Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}

// Implementation of Shape for Square
impl Shape for Square {
    fn area(&self) -> u32 {
        self.side * self.side
    }

    fn perimeter(&self) -> u32 {
        4 * self.side
    }
}

// Implementation of Shape for Circle
impl Shape for Circle {
    fn area(&self) -> u32 {
        self.radius * self.radius * 3 // Using 3 as π approximation
    }

    fn perimeter(&self) -> u32 {
        2 * 3 * self.radius // Circumference using 3 as π approximation
    }
}

// Function to get the area of any shape implementing the Shape trait
fn get_area(shape: impl Shape) -> u32 {
    shape.area()
}

// ----------------------- User Struct and Display Implementation -----------------------

// Struct representing a user with a name and age
#[derive(Debug)]
struct User {
    name: String,
    age: u32,
}

// Implementation of Display trait for User
impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.name, self.age)
    }
}

// ----------------------- Main Function -----------------------

fn main() {
    // Test serialization and deserialization of Swap
    let swap = Swap {
        qty_1: 1,
        qty_2: 2,
    };
    // Serialize the Swap instance to a vector of bytes
    let serialized = swap.serialize();
    println!("Serialized Swap: {:?}", serialized);
    // Deserialize the vector back to a Swap instance
    let deserialized = Swap::deserialize(&serialized).unwrap();
    println!("Deserialized Swap: {:?}", deserialized);

    // Test shapes
    let rect = Rect {
        width: 20,
        height: 20,
    };
    let square = Square { side: 32 };
    let circle = Circle { radius: 10 };



    // Calculate and print perimeters
    println!("Rectangle perimeter: {}", rect.perimeter());
    println!("Square perimeter: {}", square.perimeter());
    println!("Circle perimeter (circumference): {}", circle.perimeter());

        // Calculate and print areas
    println!("Rectangle area: {}", get_area(rect));
    println!("Square area: {}", get_area(square));
    println!("Circle area: {}", get_area(circle));

    // Test User struct
    let user = User {
        name: String::from("Everest"),
        age: 1,
    };
    println!("User: {}", user);
}

// ----------------------- Placeholder for Procedural Macros -----------------------

// Note: Procedural macros like #[post("/user/")] and sqlx::query! require external crates
// and are not included in this executable code.