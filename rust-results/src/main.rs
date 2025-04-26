enum Error{ 
    General(String),
}

struct User {
    name: String,
    age: u32,
}

impl User {
    fn new(name: String, age: u32) -> Result<Self, Error> {
        if name.is_empty() {
            return Err(Error::General("Name cannot be empty".to_string()));
        }
        if age == 0 {
            return Err(Error::General("Age cannot be zero".to_string()));
        }
        Ok(User { name, age })
    }
    fn display(&self) {
        println!("Name: {}, Age: {}", self.name, self.age);
    }
}

fn main() { 
   let user = User::new("John".to_string(), 30);
   match user { 
         Ok(user) => {
            println!("User created successfully user called {}", user.name);
         },
         Err(e) => {
            match e {
                Error::General(msg) => println!("Error: {}", msg),
            }
         }
   }
}
