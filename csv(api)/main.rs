extern crate csv;
// use csv::StringRecord;
#[macro_use]extern crate rocket;

#[post("/<path>")]
fn read_csv(path:&str)->String {
    let mut reader = csv::Reader::from_path(path).expect("Can't read the file");
    let mut record_data=Vec::new();//A vector to store record data
    let headers=reader.headers().expect("Can't read headers");//read headers 
    let mut headervec:Vec<String>=Vec::new();//A vector to store headers
    for i in headers{headervec.push(i.to_string());} //Store the headers in "headervec" vector
    //read the record data 
    for result in reader.records(){
        let record = result.expect("Can't read data in the file");
        for i in 0..=record.len()-1{
       record_data.push(record[i].to_string());}//Store the records as Strings in record_data vector
    }
    format!("{:?}\n{:?}",headervec,record_data)//returning the output
}

#[post("/<path>/<query>")]
fn student_name(path:&str,query:&str)->String{
    let mut reader = csv::Reader::from_path(path).expect("Can't read the file");
    let mut student_details=Vec::new();//A vector to store student_details
    for result in reader.records(){
        let record=result.expect("Can't read the record");
        if query.to_lowercase()==record[0].to_lowercase()//matching the query(student_name provided)
        {
            for i in 0..=record.len()-1{
            student_details.push(record[i].to_string());}//storing all the data of the student in student_details vector
        }
    }
    format!("{:#?}",student_details)//returning the required data of the student
}
#[launch]
fn rocket()->_{
    rocket::build()
                .mount("/",routes![read_csv])
                .mount("/student_details",routes![student_name])
}
