use chrono::{NaiveDate,NaiveDateTime,Duration,Utc, TimeZone};
use chrono::format::ParseError;

fn main() {
    let new_date = add("hour",3,"2022-02-23 12:32:33");
    println!("{:?}",new_date);
}

// #[allow(unused_variables)]
fn add(category:&str,mut input:i64,date:&str)->String{
    // let _date_res = check_date(date);
    let mut year;
    let mut month;
    let mut day=0;
    let mut hour=0;
    let mut min=0; 
    let mut sec=0; 
    let mut millisecs =0;
    if date.len() == 10 {
    let _date_res = check_date(date);
    let v:Vec<&str> = date.split(&['-','/',' '][..]).collect();
    year = v[0].parse::<i64>().unwrap();
    month = v[1].parse::<i64>().unwrap();
    day = v[2].parse::<i64>().unwrap();}
    else if date.len() == 19{
    let _date_res = check_date(date);
    let v:Vec<&str> = date.split(&['-','/',' ',':','.'][..]).collect();
    year = v[0].parse::<i64>().unwrap();
    month = v[1].parse::<i64>().unwrap();
    day = v[2].parse::<i64>().unwrap();
    hour = v[3].parse::<i64>().unwrap();
    min = v[4].parse::<i64>().unwrap();
    sec = v[5].parse::<i64>().unwrap();
    }
    else {
        let v:Vec<&str> = date.split(&['-','/',' ',':','.'][..]).collect();
    year = v[0].parse::<i64>().unwrap();
    month = v[1].parse::<i64>().unwrap();
    day = v[2].parse::<i64>().unwrap();
    hour = v[3].parse::<i64>().unwrap();
    min = v[4].parse::<i64>().unwrap();
    sec = v[5].parse::<i64>().unwrap();
    millisecs = v[6].parse::<i64>().unwrap();
    }
    match category{
        "year" | "yyy" | "y" =>{
            year = year + input;
            if year >= 10000{
                println!("Invalid input");
            }
            else if year <= 0{println!("Invalid year");}
            else{year = year;}
            let result = year.to_string()+&"-".to_string()+&month.to_string()+&"-".to_string()+&day.to_string()+&" ".to_string()+&hour.to_string()
            +&":".to_string()+&min.to_string()+&":".to_string()+&sec.to_string()+&millisecs.to_string();
            return result;
        },
        "month" | "mm" | "m" =>{
            if input >= 12{
                input = month + input;
                let a = input % 12 ;
                let b = input / 12 ;
                month = a ;
                year = year + b ;
                let result = year.to_string()+&"-".to_string()+&month.to_string()+&"-".to_string()+&day.to_string()+&" ".to_string()+&hour.to_string()
                +&":".to_string()+&min.to_string()+&":".to_string()+&sec.to_string()+&millisecs.to_string();
                return result;
            }
            else if input.to_string().contains('-'){
                if input * -1 >= month{
                    year -= 1;
                    month = month + 12 ;
                    month = month - (input * -1) ;
                    let result = year.to_string()+&"-".to_string()+&month.to_string()+&"-".to_string()+&day.to_string()+&" ".to_string()+&hour.to_string()
                    +&":".to_string()+&min.to_string()+&":".to_string()+&sec.to_string()+&millisecs.to_string();
                    return result;
                }
                else{
                    month = month - (input * -1);
                    let result = year.to_string()+&"-".to_string()+&month.to_string()+&"-".to_string()+&day.to_string()+&" ".to_string()+&hour.to_string()
                    +&":".to_string()+&min.to_string()+&":".to_string()+&sec.to_string()+&millisecs.to_string();
                    return result;
                }
            }
            else{
                month = month + input;
                let result = year.to_string()+&"-".to_string()+&month.to_string()+&"-".to_string()+&day.to_string()+&" ".to_string()+&hour.to_string()
                +&":".to_string()+&min.to_string()+&":".to_string()+&sec.to_string()+&millisecs.to_string();
                return result;}
        },
        "day" | "dy" | "dayofyear" | "d" | "dd" | "weekday" | "dw" | "w" =>{
            let dt = Utc.ymd(year.try_into().unwrap(), month.try_into().unwrap(), day.try_into().unwrap()) + Duration::days(input);
            let result = dt.to_string().replace("UTC"," ");
            return result;
        },
        "quarter" | "qq" | "q" => {
            input = input * 3;
            if input >= 12{
                input = month + input;
                let a = input % 12 ;
                let b = input / 12 ;
                month = a ;
                year = year + b ;
                let result = year.to_string()+&"-".to_string()+&month.to_string()+&"-".to_string()+&day.to_string()+&" ".to_string()+&hour.to_string()
                +&":".to_string()+&min.to_string()+&":".to_string()+&sec.to_string()+&millisecs.to_string();
                return result;
            }
            else if input.to_string().contains('-'){
                if input * -1 >= month{
                    year -= 1;
                    month = month + 12 ;
                    month = month - (input * -1) ;
                    let result = year.to_string()+&"-".to_string()+&month.to_string()+&"-".to_string()+&day.to_string()+&" ".to_string()+&hour.to_string()
                    +&":".to_string()+&min.to_string()+&":".to_string()+&sec.to_string()+&millisecs.to_string();
                    return result;
                }
                else{
                    month = month - (input * -1);
                    let result = year.to_string()+&"-".to_string()+&month.to_string()+&"-".to_string()+&day.to_string()+&" ".to_string()+&hour.to_string()
                    +&":".to_string()+&min.to_string()+&":".to_string()+&sec.to_string()+&millisecs.to_string();
                    return result;
                }
            }
            else{
                month = month + input;
                let result = year.to_string()+&"-".to_string()+&month.to_string()+&"-".to_string()+&day.to_string()+&" ".to_string()+&hour.to_string()
                +&":".to_string()+&min.to_string()+&":".to_string()+&sec.to_string()+&millisecs.to_string();
                return result;
            }
        },
        "weeks" | "ww" | "wk"  => {
            let days = (input * 7) as i64;
            // let dt = Utc.ymd(year.try_into().unwrap(), month, day) + Duration::days(days);
            let dt = Utc.ymd(year.try_into().unwrap(), month.try_into().unwrap(), day.try_into().unwrap())
            .and_hms(hour.try_into().unwrap(), min.try_into().unwrap(), sec.try_into().unwrap()) + Duration::days(days);
            let result = dt.to_string().replace("UTC"," ");
            return result;
        },
        "hour" | "hours" =>{
            let dt = Utc.ymd(year.try_into().unwrap(), month.try_into().unwrap(), day.try_into().unwrap())
                                .and_hms(hour.try_into().unwrap(), min.try_into().unwrap(), sec.try_into().unwrap()) + Duration::hours(input.into());
           let result = dt.to_string().replace("UTC"," ");
           return result;
        }
        "minutes" | "mi" | "n" =>{
            let dt = Utc.ymd(year.try_into().unwrap(), month.try_into().unwrap(), day.try_into().unwrap())
                                .and_hms(hour.try_into().unwrap(), min.try_into().unwrap(), sec.try_into().unwrap()) + Duration::minutes(input.into());
            let result = dt.to_string().replace("UTC"," ");
            return result;
        },
        "seconds" | "sec" |"s" => {
            let dt = Utc.ymd(year.try_into().unwrap(), month.try_into().unwrap(), day.try_into().unwrap())
                                .and_hms(hour.try_into().unwrap(), min.try_into().unwrap(), sec.try_into().unwrap()) + Duration::seconds(input.into());
            let result = dt.to_string().replace("UTC"," ");
            return result;
        },
        "millisecond" | "ms" => {
            let dt = Utc.ymd(year.try_into().unwrap(), month.try_into().unwrap(), day.try_into().unwrap()).and_hms_milli(hour.try_into().unwrap(), min.try_into().unwrap(), sec.try_into().unwrap(),millisecs.try_into().unwrap()) 
                                 + Duration::milliseconds(input.into());
            let result = dt.to_string().replace("UTC"," ");
            return result;
        },
        _ => todo!(),
    }
    
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

