## Code #1

Read <a href="main.md">Main.md</a> for more details and solutions to all the problems in this code!

```rust 
enum Role { 
    Client,
    Admin,
    Guest,
}
enum Course <Vector>{ 
    Rust(Option<Vec<String>, None>),
    Html(Vec<String>),
    Java(Vec<String>),
}
struct User { 
  name: String, 
  role: Role,
  class: Courses,
}

enum Modules { 
    Rust(String),
    Html(String),
    Java(String),
}

impl User { 

    fn add_course(&mut self, course: Course, modules: Modules) -> Course<String> { 
        let mut user_courses = Vec::new();
        match self.course { 
            Course::Rust => user_courses.push(match modules {
                Modules::Rust("Rust for beginners") => ,
                _ => String::from(""),
            }),
            Courses::Html => user_courses.push(String::from("Html")),
            Courses::Java => user_courses.push(String::from("Java")),
        }
        // match self.class
        user_courses
    }
    fn create(name: String, role: Role, class: Course) -> Self{ 
        Self { 
          name, 
          role,
          class,
        }
    }


    fn get_name(&self) -> &String { 
        &self.name
    }
    fn get_role(&self) -> String { 
        match self.role {
            Role::Client => String::from("Client"),
            Role::Admin => String::from("Admin"),
            Role::Guest => String::from("Guest"),
        }
    }
    fn get_user(&self){ 
      println!("Name: {}", self.get_name());
      println!("{}, is a {} user", self.get_name(), self.get_role());
    }
}

fn main() { 
}

```