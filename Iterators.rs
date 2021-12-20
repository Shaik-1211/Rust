#[derive(Debug)]
struct Shoe{
    size:u32,
    kind:String
}

//A function to check if there are any kind of shoes available with the given shoe size.
fn in_size(shoe:Vec<Shoe>,required_size:u32) -> Vec<Shoe> {
    shoe.into_iter().filter(|s| s.size==required_size).collect()
}

fn main() {
    let shoes=vec![
        Shoe{size:10, kind:String::from("Sneakers")},
        Shoe{size:9, kind:String::from("Loafer")},
        Shoe{size:11, kind:String::from("Sandal")},
        Shoe{size:10, kind:String::from("Ballet flats")},
        Shoe{size:7, kind:String::from("Boot")},
    ];
    let in_my_size=in_size(shoes,10);
    for i in &in_my_size{
        println!("{:?}",i);//prints the available kinds of shoes with the given shoe size.
    }
}