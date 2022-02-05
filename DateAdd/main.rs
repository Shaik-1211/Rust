use chrono::{NaiveDate,NaiveDateTime,Duration,Utc, TimeZone};
use chrono::format::ParseError;

fn main() {
    let new_date = add("ms",23,"2022-02-23 12:43:40.98");
    let x = new_date.0.to_string()+&"-".to_string()+&new_date.1.to_string()+&"-".to_string()+&new_date.2.to_string();
    let n_date = NaiveDate::parse_from_str(x.as_str(), "%Y-%m-%d");
    match n_date{
        Ok(t)=> println!("new date is {}",t),
        Err(e)=> println!("{}",e),
    }
}

// #[allow(unused_variables)]
fn add(category:&str,mut input:u32,date:&str)->(u32,u32,u32) {
    let _date_res = check_date(date);
    let mut year;
    let mut month;
    let mut day=0;
    let mut hour=0;
    let mut min=0; 
    let mut sec=0; 
    let mut millisecs =0;
    if date.len() == 10 {
    let v:Vec<&str> = date.split(&['-','/',' '][..]).collect();
    year = v[0].parse::<u32>().unwrap();
    month = v[1].parse::<u32>().unwrap();
    day = v[2].parse::<u32>().unwrap();}
    else {
        let v:Vec<&str> = date.split(&['-','/',' ',':','.'][..]).collect();
    year = v[0].parse::<u32>().unwrap();
    month = v[1].parse::<u32>().unwrap();
    day = v[2].parse::<u32>().unwrap();
    hour = v[3].parse::<u32>().unwrap();
    min = v[4].parse::<u32>().unwrap();
    sec = v[5].parse::<u32>().unwrap();
    millisecs = v[6].parse::<u32>().unwrap();
    }
    
    match category{
        "year" | "yyy" | "y" =>{
            year = year + input;
            if year >= 10000{
                println!("Invalid input");
            }
            else{year = year;}
        },
        "month" | "mm" | "m" =>{
            if input >= 12{
                input = month + input;
                let a = input % 12 ;
                let b = input / 12 ;
                month = a ;
                year = year + b ;
            }
            else{month = month + input;}
        },
        "day" | "dy" | "dayofyear" | "d" | "dd" | "weekday" | "dw" | "w" =>{
            let dt = Utc.ymd(year.try_into().unwrap(), month, day) + Duration::days(123);
            println!("added date is {:?}", dt.to_string().replace("UTC"," "));
        },
        "quarter" | "qq" | "q" => {
            input = input * 3;
            if input >= 12{
                input = month + input;
                let a = input % 12 ;
                let b = input / 12 ;
                month = a ;
                year = year + b ;
            }
            else{month = month + input;}
        },
        "weeks" | "ww" | "wk"  => {
            let days = (input * 7) as i64;
            let dt = Utc.ymd(year.try_into().unwrap(), month, day) + Duration::days(days);
            println!("added date is {:?}", dt.to_string().replace("UTC"," "));
        },
        "hour" =>{
            let dt = Utc.ymd(year.try_into().unwrap(), month, day).and_hms(hour, min, sec) + Duration::hours(input.into());
            
            println!("{:?}", dt);
        }
        "minutes" | "mi" | "n" =>{
            let dt = Utc.ymd(year.try_into().unwrap(), month, day).and_hms(hour, min, sec) + Duration::minutes(input.into());
            println!("{:?}", dt);
        },
        "seconds" | "sec" |"s" => {
            let dt = Utc.ymd(year.try_into().unwrap(), month, day).and_hms(hour, min, sec) + Duration::seconds(input.into());
            println!("{:?}", dt);
        },
        "millisecond" | "ms" => {
            let dt = Utc.ymd(year.try_into().unwrap(), month, day).and_hms_milli(hour, min, sec,millisecs) 
                                 + Duration::milliseconds(input.into());
            println!("{:?}", dt);
        },
        _ => todo!(),
    }
    (year,month,day)
}

fn check_date(date_only:&str)->Result<(),ParseError>{
    if date_only.len() == 10 {
     if let  Err(e) = NaiveDate::parse_from_str(date_only, "%Y-%m-%d"){
        println!(" kk {} ",e);
     }}
     else{
         if let Err(e) = NaiveDateTime::parse_from_str(date_only, "%Y-%m-%d %H:%M:%S"){
             println!(" {} ",e);
         }
     }
     Ok(())
}

