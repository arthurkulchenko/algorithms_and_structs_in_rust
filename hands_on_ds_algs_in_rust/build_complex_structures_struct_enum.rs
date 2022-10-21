pub struct Person {
  name:String,
  age:i32,
  children:i32
}

impl Person {
  pub fn represent(self) -> String {
    format!("name: {}, age: {}, children: {}", self.name, self.age, self.children)
  }
}

fn main() {
  let person = Person{name: "Mat".to_string(), age: 35, children: 4 }
  println!("Hello {}", person.represent);
}
