extern crate csv;
use std::error::Error;

// #[warn(unused_assignments)]
// #[warn(unused_allocation)]
fn write_csv(inputpath: &str,outputpath:&str)->Result<(),Box<dyn Error>>{
    let mut rdr=csv::Reader::from_path(inputpath)?;
    let mut wtr=csv::Writer::from_path(outputpath)?;
    wtr.write_record(&["first_name","Last_name","Fullname"])?;
    // let mut full_name=String::new();
    // let mut first_name=String::new();
    // let mut last_name=String::new();
    for result in rdr.records() {
        let record=result?;
        // first_name=record[0].to_string();
        // last_name=record[1].to_string();
        // full_name=record[0].to_string()+" "+&record[1].to_string();
    // wtr.write_record(&csv::StringRecord::from(vec![first_name, last_name,full_name]))?;
    wtr.write_record(&csv::StringRecord::from(
        vec![record[0].to_string(),record[1].to_string(),record[0].to_string()+" "+&record[1].to_string()]))?;
    }
    Ok(())
}

fn main() {
    if let Err(e)=write_csv("input.csv","output.csv"){
        println!("{:?}",e);
    }
}