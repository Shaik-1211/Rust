#[macro_use]extern crate rocket;

#[get("/<a>/<b>")]
fn add(a:i32,b:i32)->String{
    let c=a+b;
    format!("The sum of {} and {} is {}",a,b,c)
}
#[get("/<name>/<age>/<cool>")]
fn communicate(name:&str,age:u32,cool:bool)->String{
    if cool
    {format!("Hey {}, a {} years old cool buddy!",name,age)}
    else
    {format!("Hi!, Just chill {} year old {}",age,name)}
}

#[launch]
fn rocket()->_{
    rocket::build()
                .mount("/",routes![add])
                .mount("/hello", routes![communicate])
}
