extern crate csv;//External crate {csv="1.1"} has been added in cargo.toml file
extern crate serde;//External crate {serde="1"} has been added in cargo.toml file
#[macro_use]
extern crate serde_derive;//External crate {serde_derive="1"} has been added in cargo.toml file
use std::io;
use std::error::Error;
use CSV;
mod lib;

#[derive(Debug,Deserialize)]
#[serde(rename_all="PascalCase")]
pub struct Record 
{   
    #[serde(rename = "Student_Name")]    
    pub studentname:String,
    #[serde(rename = "Student_Id")]
    pub studentid:i64,
    #[serde(rename = "English_Score")]
    pub english_score:Option<i64>,
    #[serde(rename = "Maths_Score")]
    pub maths_score:Option<i64>,
   #[serde(rename = "Science_Score")]
    pub science_score:Option<i64>,
   #[serde(rename = "Social_Score")]
    pub social_score:Option<i64>,
    #[serde(rename="Computer_Score")]
    pub computer_score:Option<i64>,
}
pub fn read_csv(path:&str,name:String)->Result<(),Box<dyn Error>>
{
    let mut reader=csv::Reader::from_path(path)?;
    let _headers=reader.headers()?;
    //println!("{:#?}",headers);//Only prints the header of the csv file
    let mut count=0;
    for result in reader.deserialize()
    {   
        let record:Record=result?;
        
        //Prints the record of the student name if input name (entered by the user) is matched with one of the name in the file. 
        if record.studentname.to_lowercase() == name
        {println!("{:#?}",record);
        break;}
        else 
        { count+=1;
        }
        //println!("{:#?}",record);//This will print all the data in the csv file
        
    }
    if count>=15
    {println!("No record found for the student_name entered.");}
    Ok(())
}

pub fn search_through_id(path:&str,id:i64)->Result<(),Box<dyn Error>>
{
    let mut reader=csv::Reader::from_path(path)?;
    let mut count=0;
    for result in reader.deserialize()
    {   
        let record:Record=result?;
        if record.studentid == id
        {println!("{:#?}",record);
        break;}
        else 
        { count+=1;
        }
        
    }
    if count>15
    {println!("No record found for the student_id entered.");}
    Ok(())
}

pub fn null_count(path:&str)->Result<(),Box<dyn Error>>
{
    let mut reader=csv::Reader::from_path(path)?;
    let mut count=0;
    let mut count_english=0;
    let mut count_maths=0;
    let mut count_science=0;
    let mut count_social=0;
    let mut count_computer=0;

    for result in reader.deserialize()
    {let record:Record=result?;
        if record.english_score==None
        {count_english+=1;}
        if record.maths_score==None
        {count_maths+=1;}
        if record.science_score==None
        {count_science+=1;}
        if record.social_score==None
        {count_social+=1;}
        if record.computer_score==None
        {count_computer+=1;}
    }
    println!("Total null values in english_score = {:?}",count_english);
    println!("Total null values in maths_score = {:?}",count_maths);
    println!("Total null values in science_score = {:?}",count_science);
    println!("Total null values in social_score = {:?}",count_social);
    println!("Total null values in computer_score = {:?}",count_computer);

    Ok(())
}
fn main()
{   
    let mut chances=5;
    loop
  {
    println!("Choose the action to perform");
    println!("
    1.Get a record of a student(use student name)
    2.Get a record of a student(use student id)     
    3.Display a column  
    4.Check whether a record has repeated
    5.print the total number of fields in each column 
    6.Print the null values in each column");
    let mut option=String::new();
    io::stdin().read_line(&mut option).expect("invalid input");
    let choice:i32=option.trim().parse().expect("Enter a valid option number");

    if choice==1{
        println!("Enter the student name to get the record of the student");
        let mut a=String::new();
        io::stdin().read_line(&mut a).expect("invalid input for choice1");
        let name=a.trim().to_lowercase();
        if let Err(e)=read_csv("studentinfo.csv",name)
        {eprintln!("Error is {}",e);}
        break;}

    else if choice==2
    {   let id=CSV::get_id();
        match id

        {
            Ok(num)=>if let Err(e)=search_through_id("studentinfo.csv", num){
                println!("{:?}",e);},
                Err(e)=>{println!("Error getting id {:?}",e);}
        }
        
    break;}
    else if choice==3{
    let column=CSV::choice2();
    match column
    {
            Ok(column_name)=>if let Err(e)=CSV::get_column("studentinfo.csv",column_name){
                println!("{:?}",e);},
        Err(e)=>{println!("Error occured in choice2 {:?}",e);},
    }
        break;}


    else if choice==4{
    if let Err(e)=CSV::check("studentinfo.csv"){
        println!("{:?}",e);
    }break;}
    
    // else if choice==5{
    //     if let Err(e)=write("studentinfo.csv"){
    //         println!("Error writing in csv {:?}",e);
    //     }break;
    // }

    else if choice==5
    {
        if let Err(e)=CSV::count("studentinfo.csv"){
            println!("Error while counting duplicate data {:?}",e);
        }
        break;
    }

    else if choice==6
    {
        if let Err(e)=null_count("studentinfo.csv"){
            println!("Error while count null vaues : {:?}",e);
        }
        break;
    }
    else {println!("Choose from the above 5 options");
    
    if chances==1{break;}
    chances-=1;}
  }
}

