use std::io;
use std::collections::HashSet;
extern crate csv;
extern crate serde;
use std::error::Error;

fn distinct_data(path:&str,query:i32)->Result<(),Box<dyn Error>>
{   let mut reader=csv::Reader::from_path(path)?;
    let mut distinct_values=Vec::new();
    let mut values=HashSet::new();
    
    for result in reader.records(){
        let record=result?;
        if query==1{
        let name=record[0].to_string();
            values.insert(name);
        }
        if query==2 {
        let id=record[1].to_string();
            values.insert(id);
        }
        if query== 3{
        let english_score=record[2].to_string();
            values.insert(english_score);
        }
        if query==4 {
        let name=record[3].to_string();
            values.insert(name);
        }
        if query== 5{
        let science_score=record[4].to_string();
            values.insert(science_score);
        }
        if query==6{
        let social_score=record[5].to_string();
            values.insert(social_score);
        }
        if query==7{
        let computer_score=record[6].to_string();
            values.insert(computer_score);
        }
    }

    for item in &values{
            distinct_values.push(item);
            // println!("{:?}",item);//Prints the items in HashSet in arbitary order
        }
        println!("Distinct values present in the selected column are :");
        distinct_values.sort();
        for element in &distinct_values{
            print!("{:?}  ",element);
        }
       
    Ok(())
}

fn get_duplicate_data(path:&str)->Result<(),Box<dyn Error>>
{
    let mut reader = csv::Reader::from_path(path)?;
    let mut duplicate_data:Vec<String>=Vec::new();
    for result in reader.records()
    {
        let _record=result?;
        let name=_record[0].to_string();
        duplicate_data.push(name);
    }

    let mut _var1=false;
    let mut i=1;
    let mut _var2=Vec::new();
    for name in &duplicate_data
        {
            for j in &duplicate_data[i..]
            { if name==j {_var1=true;
                          _var2.push(name);}
                else {_var1=false;}
            }
            i+=1;
        }
        if _var2.is_empty()==false
        {println!("Duplicate data found and the student_name of the records repeated : {:?}",_var2); }
        else
        {println!("No Duplicate data found");}
    Ok(())
}

fn main()
{
    println!("Choose the column whose distinct values to be printed :");
    println!("1.student_name 2.student_id 3.English_score 
    4.Maths_score 5.Science_score 6.Social_score 7.Computer_score ");

    let mut choice=String::new();
    io::stdin().read_line(&mut choice).expect("Enter a valid input ");
    let choice=choice.trim().parse().expect("Enter a valid input");
    if let Err(e)=distinct_data("student_data.csv",choice){
        println!("{:?}",e);
    }

    if let Err(e)=get_duplicate_data("student_data.csv"){
        println!("{:?}",e);
    }

}