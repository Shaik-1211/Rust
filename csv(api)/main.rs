extern crate csv;
use csv::StringRecord;
#[macro_use]extern crate rocket;

#[post("/<path>")]
fn read_csv(path:&str) {
    let mut reader = csv::Reader::from_path(path).expect("Can't read the file");
    let mut record_data:Vec<StringRecord>=Vec::new();
    let headers=reader.headers().expect("Can't read headers");
    let mut headervec:Vec<String>=Vec::new();
    for i in headers{headervec.push(i.to_string());} 
    for result in reader.records(){
        let record = result.expect("Can't read the file");
       record_data.push(record);
    }
    for h in headervec{println!("{:?}",h);}
    for data in record_data{println!("{:?}",data);}
}

#[launch]
fn rocket()->_{
    rocket::build().mount("/",routes![read_csv])
}