extern crate csv;//External crate {csv="1.1"} has been added in cargo.toml file
extern crate serde;//External crate {serde="1"} has been added in cargo.toml file
#[macro_use]
extern crate serde_derive;//External crate {serde_derive="1"} has been added in cargo.toml file
use std::error::Error;

#[derive(Debug,Deserialize)]
#[serde(rename_all="PascalCase")]
struct Record 
{   
    #[serde(rename = "Student_Name")]    
    studentname:String,
    #[serde(rename = "Student_Id")]
    studentid:i64,
    #[serde(rename = "English_Score")]
    english_score:i64,
    #[serde(rename = "Maths_Score")]
    maths_score:i64,
   #[serde(rename = "Science_Score")]
    science_score:i64,
   #[serde(rename = "Social_Score")]
    social_score:i64,
    #[serde(rename="Computer_Score")]
    computer_score:i64,
}
fn read_header(path:&str)->Result<(),Box<dyn Error>>
{
    let mut reader=csv::Reader::from_path(path)?;
    let headers=reader.headers()?;
   //println!("{:#?}",headers);//Only prints the header of the csv file

    for result in reader.deserialize()
    {
        let record:Record=result?;
        if record.studentname == "Emanuel"//This only prints the data of the student name provided here
        {println!("{:#?}",record);}
        // println!("{:#?}",record);//This will prints all the data in the csv file
    }
    Ok(())
}
fn main()
{   
    if let Err(e)=read_header("student_information.csv")
    {eprintln!("Error is {}",e);}
}