use yew::prelude::*; 

 enum Gender { 
    Male, 
    Female,
 }

 struct User { 
    name: String, 
    age: i32,
    gender: Gender,
 }
#[function_component(App)]
fn app() -> Html { 
    html! { 
      <p>{"Milton is awesome"}</p>
    }
}

impl User { 
   fn create(user_name: String, user_age: i32, user_gender: Gender) -> Self { 
     Self {
        name: user_name,
        age: user_age, 
        gender: user_gender,
     }
   }
   fn show_user(user: &Self){
   
   }
}

fn main() { 
    yew::Renderer::<App>::new().render();
}