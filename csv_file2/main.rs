extern crate csv;
use std::error::Error;

fn write_csv(inputpath: &str,outputpath:&str)->Result<(),Box<dyn Error>>{
    let mut rdr=csv::Reader::from_path(inputpath)?;
    let mut wtr=csv::Writer::from_path(outputpath)?;
    wtr.write_record(&["first_name","Last_name","Fullname","Dot extension","Length of dot extension"])?;//writes header in ouput.csv
    for result in rdr.records() {
        let record=result?;
        let mut first_name=record[0].to_string();
        first_name.push('.');
        let mut var=String::new();
        let (first,last)=first_name.split_at(1);//splitting the first name at index 1 
        var.push_str(first);
        first_name.push_str(var.as_str());//Adding the first letter of the full name
        println!("{:?}",first_name);
        let length=first_name.len().to_string();//Calculating the length of Dot extension
        //writing the data to the output csv file
    wtr.write_record(&csv::StringRecord::from(
        vec![record[0].to_string(),record[1].to_string(),record[0].to_string()+" "+&record[1].to_string(),first_name,length]))?;
    }
    Ok(())
}

fn main() {
    if let Err(e)=write_csv("input.csv","output.csv"){
        println!("{:?}",e);
    }
}
