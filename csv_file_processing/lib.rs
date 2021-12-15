extern crate csv;//External crate {csv="1.1"} has been added in cargo.toml file
extern crate serde;//External crate {serde="1"} has been added in cargo.toml file
// #[macro_use]
// extern crate serde_derive;//External crate {serde_derive="1"} has been added in cargo.toml file
use std::error::Error;
// use std::process;
use std::io;

pub fn count(path:&str)->Result<(),Box<dyn Error>>
{
    let mut reader=csv::Reader::from_path(path)?;
    let mut store:Vec<String>=Vec::new();
    let mut size=0;
    //This loop will iterate over the records in csv file 
    for result in reader.records()
    { let record=result?;
        let  mut student_data=String::new();
        student_data=record[0].to_string();
        //Storing the fields of the column in a vector
        store.push(student_data);
        size=record.len();
    
    }
    let mut reader=csv::Reader::from_path(path)?;
    //reading the headers of the csv file
    let headers=reader.headers()?;
    // println!("record size is {:?}",size);

    //This loop will print the header name and the total number of fields under that header
    for i in 0..=size-1
    {   
             println!("{:?} : {:?}",&headers[i],store.len());
    }

    Ok(())
}

//This will match the given column name and print the data in the given column
pub fn get_column(path:&str,query:String)->Result<(),Box<dyn Error>>
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
            // match english_score{
            //     Some(score)=>println!("english_score:{:?}",english_score),
            //     None(no_value)=>println!("None"),
            // }
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
pub fn check(path:&str)->Result<(),Box<dyn Error>>
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

pub fn headers(path:&str)->Result<(),Box<dyn Error>>
{
    let mut reader=csv::Reader::from_path(path)?;
    let headers=reader.headers()?;
    println!("{:?}",&headers[0]);
   println!("{:#?}",headers);//Only prints the header of the csv file
   Ok(())
}

pub fn choice2()->Result<String,Box<dyn Error>>
{
    println!("The columns in the file 'studentinfo.csv' are : ");
    if let Err(e)=headers("studentinfo.csv"){
        println!("Error occured {:?}",e);
    }
    println!("Enter the column name you need to be displayed:");
    let mut b=String::new();
    io::stdin().read_line(&mut b)?;
    let column=b.trim().to_lowercase();
    Ok(column)

}
 
pub fn get_id()->Result<i64,Box<dyn Error>>
{   println!("Enter Student Id : ");
    let mut c=String::new();
    io::stdin().read_line(&mut c)?;
    let num:i64=c.trim().parse().expect("Enter an integer");

    Ok(num)
}

pub fn write(path:&str)->Result<(),Box<dyn Error>>
{
    let mut wtr=csv::Writer::from_path(path)?;
    wtr.write_record(&["umar","117","56","74","87","94"])?;
    Ok(())
}

