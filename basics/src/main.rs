#[derive(Debug)]
enum Form {
    FirstYear(Subjects),
    SecondYear(Subjects),
    ThirdYear(Subjects),
}

#[derive(Debug)]
enum Subjects {
    Maths(Modules, Modules),
    English(Modules, Modules),
    Geo(Modules),
}

#[derive(Debug)]
enum Modules {
    Introduction(String),
    Theory(String),
    Other(String),
}

struct Student {
    name: String,
    form: Form,
}

impl Student {

 // OLD CODE!
 /*    fn add_form(form: i32, subject: Subjects, user_module: Modules) -> Form {
        if form > 3 {
            println!("We only have three forms!");
        } else {
            match form {
                1 => {
                    let user_form: Form =
                        Form::FirstYear(subject::user_module(String::from(subject)));
                }
                2 => {
                    let user_form: Form =
                        Form::SecondYear(subject::user_module(String::from(subject)));
                }
                3 => {
                    let user_form: Form =
                        Form::ThirdYear(subject::user_module(String::from(subject)));
                }
                _ => {
                    println!("Lol I love pizza");
                }
            }
        }

        user_form
    }
    */
fn add_form(form: i32, subject: Subjects) -> Form {
    match form {
        1 => Form::FirstYear(subject),
        2 => Form::SecondYear(subject),
        3 => Form::ThirdYear(subject),
        _ => {
            println!("We only have three forms!");
            panic!("Invalid form number");
        }
    }
}
 // OLD CODE!
 /*    pub fn create_student(
        name: String,
        form: i32,
        subject: Subjects,
        user_module: Modules,
    ) -> Student {
        let form: Form = add_form(form, subject, user_module);
        Student { name, form }
    } */

        pub fn create_student(name: String, form: i32, subject: Subjects) -> Student {
        let form = Student::add_form(form, subject); // Call `add_form` as a static method
        Student { name, form }
    }

    // OLD CODE!
    /* fn print_student_info(&self) {
        println!("Student name is {}", &self.name);
        println!("Student is doing {:?}", &self.form);
    } */

    fn print_student_info(&self) {
        println!("Student name is {}", &self.name);
        match &self.form {
            Form::FirstYear(subject) => println!("Student is in First Year studying {:?}", subject),
            Form::SecondYear(subject) => println!("Student is in Second Year studying {:?}", subject),
            Form::ThirdYear(subject) => println!("Student is in Third Year studying {:?}", subject),
        }
    }
}
/* 
fn main() {
    let name: String = String::from("Milton");
    let form: i32 = 1;
    let subject: Subjects = Subjects::Maths((),());
    let user_module: Modules = Modules::Theory({});
    let first_student = Student::create_student(name, form, subject);
    Student::print_student_info(&first_student);
} */

// Corrected main function

fn main() {
    let name: String = String::from("Milton");
    let form: i32 = 1;

    // Correctly initialize Subjects with valid Modules
    let subject: Subjects = Subjects::Maths(
        Modules::Introduction(String::from("Basic Algebra")),
        Modules::Theory(String::from("Advanced Calculus")),
    );

    // Correctly initialize a single Module (if needed)
    let user_module: Modules = Modules::Theory(String::from("Some Theory"));

    // Create a student and print their info
    let first_student = Student::create_student(name, form, subject);
    first_student.print_student_info();
}