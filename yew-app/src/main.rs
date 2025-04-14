use yew::{prelude::*, virtual_dom::VNode};

struct User {
    id: u32,
    name: String,
}
struct Components { 
    comp: Html,
}

struct HtmlElement { 
    element: Html,
}

#[function_component(App)]

fn app() -> Html { 
    let button = Components {  comp: html! { 
    <button onclick={greet()}>{"Click Me!"}</button>
    } 
    };
    let milton = User::create(1, "Milton".to_owned());
    let comp1 =  Components::_component(button.comp);
    html! { 
    <>
    {Components::display(&comp1)}
    </>
    }
}

fn greet() -> Html {
    html! { 
       <p>{ "Hello there "}</p>
    }
}
fn main(){ 
    yew::Renderer::<App>::new().render();
}
impl User { 
    fn create(user_id: u32, user_name: String) -> Self { 
        Self { 
            id: user_id, 
            name: user_name, 
        }
    }
    fn print(person: &Self) -> Html { 
        html! { 
            <> 
              <p>{"The person Id is "} { person.id } </p>
            </>
       }
    }
}
impl Components { 
    fn _component(component_name: VNode) -> Self { 
        Self {  
           comp: component_name,
        }
    }
    fn display(full_component: &Self) -> Html { 
        full_component.comp.clone()
    }
}