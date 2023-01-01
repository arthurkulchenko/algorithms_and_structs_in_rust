#[derive(Debug)]
pub struct Person {
  name:String,
  age:i32,
  children:i32,
  favorite_color: Color
}

#[derive(Debug)]
enum Color {
    Red(String),
    Green,
    Blue,
}

impl Person {
  pub fn represent(&self) -> String {
    return format!("name: {}, age: {}, children: {}, color: {:?}", self.name, self.age, self.children, self.favorite_color);
  }
}

fn main() {
  let color = Color::Red("Hello red".to_string());
  let person = Person{name: "Mat".to_string(), age: 35, children: 4, favorite_color: color };
  // match color {
  //     Color::Red(s) => println!("red {}", s),
  //     Color::Blue => println!("Blued"),
  //     Color::Green => println!("Green"),
  // }
  println!("Hello {}", &person.represent());
  println!("Hello {:?}", person);
  
}
