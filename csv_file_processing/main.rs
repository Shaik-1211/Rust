extern crate csv;//External crate {csv="1.1"} has been added in cargo.toml file
extern crate serde;//External crate {serde="1"} has been added in cargo.toml file
#[macro_use]
extern crate serde_derive;//External crate {serde_derive="1"} has been added in cargo.toml file
use std::error::Error;
use std::process;
use std::io;

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
fn read_csv(path:&str,name:String)->Result<(),Box<dyn Error>>
{
    let mut reader=csv::Reader::from_path(path)?;
    let headers=reader.headers()?;
    //println!("{:#?}",headers);//Only prints the header of the csv file
    let mut count=0;
    for result in reader.deserialize()
    {   
        let record:Record=result?;
        
        //Prints the record of the student name if input name (entered by the user) is matched with one of the name in the file. 
        if record.studentname.to_lowercase() == name
        {println!("{:#?}",record);
        break;}
        // println!("{:#?}",record);//This will print all the data in the csv file
        else 
        { count+=1;
        }
        
    }
    if count>1
    {println!("No record found for the student_name entered.");}
    Ok(())
}

fn search_through_Id(path:&str,Id:i64)->Result<(),Box<dyn Error>>
{
    let mut reader=csv::Reader::from_path(path)?;
    let mut count=0;
    for result in reader.deserialize()
    {   
        let record:Record=result?;
        if record.studentid == Id
        {println!("{:#?}",record);
        break;}
        else 
        { count+=1;
        }
        
    }
    if count>1
    {println!("No record found for the student_id entered.");}
    Ok(())
}

fn headers(path:&str)->Result<(),Box<dyn Error>>
{
    let mut reader=csv::Reader::from_path(path)?;
    let headers=reader.headers()?;
   println!("{:#?}",headers);//Only prints the header of the csv file
   Ok(())
}

// The get_column function will print the column (choosen by the user) of the csv file.
fn get_column(path:&str,query:String)->Result<(),Box<dyn Error>>
{
    let mut rdr=csv::Reader::from_path(path)?;

    for result in rdr.records()
    {
        let record = result?;
        let student_name=&record[0];
        if query=="student_name"{
        println!("student_name:{:?}",student_name);}
        let student_id=&record[1];
        if query=="student_id"{
        println!("student_Id:{:?}",student_id);}
        let english_score=&record[2];
        if query=="english_score"{
            println!("english_score:{:?}",english_score);
        }
        let maths_score=&record[3];
        if query=="maths_score"{
            println!("maths_score:{:?}",maths_score);
        }
        let science_score=&record[4];
        if query=="science_score"{
            println!("science_score:{:?}",science_score);
        }
        let social_score=&record[5];
        if query=="social_score"{
            println!("social_score:{:?}",social_score);
        }
        let computers_score=&record[6];
        if query=="computers_score"{
            println!("computers_score:{:?}",computers_score);
        }
    }
    Ok(())
}

// This check function will check if there is any duplicated record and print the student name of the duplicated record.
fn check(path:&str)->Result<(),Box<dyn Error>>
{
    let mut rdr = csv::Reader::from_path(path)?;
    let mut store:Vec<String>=Vec::new();
    for result in rdr.records()
    {   
        let record=result?;
        let  mut student_name=String::new();
        student_name=record[0].to_string();
        store.push(student_name);
    }
    let mut var1=false;
    let mut i=1;
    let mut var2=Vec::new();

    for name in &store
        {
            for j in &store[i..]
            { if name==j {var1=true;
                          var2.push(j);}
                else {var1=false;}
            }
            i+=1;
        }
        if var1==true
        {println!("Duplicate data found and the student_name of the records repeated : {:?}",var2); }
        else
        {println!("No Duplicate data found");}
    Ok(())
}

fn choice1()->Result<(),Box<dyn Error>>
{   println!("Enter the student name to get the record of the student");
    let mut a=String::new();
    io::stdin().read_line(&mut a)?;
    let name=a.trim().to_lowercase();
    if let Err(e)=read_csv("studentinfo.csv",name)
    {eprintln!("Error is {}",e);}
    Ok(())
}

fn choice2()->String
{
    println!("The columns in the file 'studentinfo.csv' are : ");
    if let Err(e)=headers("studentinfo.csv"){
        println!("Error occured {:?}",e);
    }
    println!("Enter the column name you need to be displayed:");
    let mut b=String::new();
    io::stdin().read_line(&mut b);
    let column=b.trim().to_lowercase();
    column

}
 
fn main()
{    let mut chances=5;
    loop
  {
    println!("Choose the action to perform");
    println!("1.Get a record of a student(use student name)  2.1.Get a record of a student(use student id)   3.Display a column   4.Check whether a record has repeated");
    let mut option=String::new();
    io::stdin().read_line(&mut option);
    let choice:i32=option.trim().parse().expect("Enter a valid option number");

    if choice==1{choice1();
    break;}

    else if choice==2
    {   println!("Enter Student Id : ");
        let mut c=String::new();
        io::stdin().read_line(&mut c);
        let num:i64=c.trim().parse().expect("Enter an integer");
        if let Err(e)=search_through_Id("studentinfo.csv", num){
        println!("{:?}",e);
    }break;}
    else if choice==3{
       let column_name=choice2();
       if let Err(e)=get_column("studentinfo.csv",column_name){
        println!("{:?}",e);
        }break;}


    else if choice==4{
    if let Err(e)=check("studentinfo.csv"){
        println!("{:?}",e);
    }break;}

    else {println!("Choose from the above 3 options");

    if chances==1{break;}
    chances-=1;}
  }
}



