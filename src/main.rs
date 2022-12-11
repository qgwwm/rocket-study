use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Apple<'a>{
    color: &'a str,
    weight: i32
}


#[macro_use] extern crate rocket;

#[get("/")]
fn index()-> &'static str{
    "hello world"
}

#[launch]
fn rocket()-> _{
    rocket::build().mount("/",routes![index])
}

// fn main() {
//     let apple = Apple{color: "red", weight: 32};
//     let ser = serde_json::to_string(&apple).unwrap();
//     println!("the ser apple is :{}",ser);
//     let de: Apple = serde_json::from_str(&ser).unwrap();
//     println!("the de apple is {:?}",de);
// }
