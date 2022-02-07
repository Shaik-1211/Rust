// use chrono::{NaiveDate,NaiveDateTime,Duration,Utc, TimeZone};
// use chrono::format::ParseError;
pub mod add_date_time{
    use std::string::ParseError;

    use chrono::{NaiveDateTime, Duration, Utc, TimeZone, NaiveDate};

// #[allow(unused_variables)]
pub fn add(category:&str,mut input:i64,date:&str)->String{
    let (mut year, mut month, day, hour, min, sec, millisecs) = get_date_time(date);
    match category{
        "year" | "yyy" | "y" =>{
            year = year + input;
            if year >= 10000{
                println!("Invalid input");
            }
            else if year <= 0{println!("Invalid year");}
            else{year = year;}
            let result = year.to_string()+&"-".to_string()+&month.to_string()+&"-".to_string()+&day.to_string()+&" ".to_string()+&hour.to_string()
            +&":".to_string()+&min.to_string()+&":".to_string()+&sec.to_string();
            let dt = NaiveDateTime::parse_from_str(&result, "%Y-%m-%d %H:%M:%S");
            match dt {
                Ok(dt) => dt.to_string(),
                Err(e)=>e.to_string(),
            }
            // return dt;
        },
        "month" | "mm" | "m" =>{
            if input >= 12{
                input = month + input;
                let a = input % 12 ;
                let b = input / 12 ;
                month = a ;
                year = year + b ;
                let result = year.to_string()+&"-".to_string()+&month.to_string()+&"-".to_string()+&day.to_string()+&" ".to_string()+&hour.to_string()
                +&":".to_string()+&min.to_string()+&":".to_string()+&sec.to_string();
                let dt = NaiveDateTime::parse_from_str(&result, "%Y-%m-%d %H:%M:%S");
                match dt {
                Ok(dt) => dt.to_string(),
                Err(e)=>e.to_string(),
            }
            }
            else if input.to_string().contains('-'){
                if input * -1 >= month{
                    year -= 1;
                    month = month + 12 ;
                    month = month - (input * -1) ;
                    let result = year.to_string()+&"-".to_string()+&month.to_string()+&"-".to_string()+&day.to_string()+&" ".to_string()+&hour.to_string()
                +&":".to_string()+&min.to_string()+&":".to_string()+&sec.to_string();
                let dt = NaiveDateTime::parse_from_str(&result, "%Y-%m-%d %H:%M:%S");
                match dt {
                Ok(dt) => dt.to_string(),
                Err(e)=>e.to_string(),
            }
                }
                else{
                    month = month - (input * -1);
                    let result = year.to_string()+&"-".to_string()+&month.to_string()+&"-".to_string()+&day.to_string()+&" ".to_string()+&hour.to_string()
                +&":".to_string()+&min.to_string()+&":".to_string()+&sec.to_string();
                let dt = NaiveDateTime::parse_from_str(&result, "%Y-%m-%d %H:%M:%S");
                match dt {
                Ok(dt) => dt.to_string(),
                Err(e)=>e.to_string(),
            }
                }
            }
            else{
                month = month + input;
                let result = year.to_string()+&"-".to_string()+&month.to_string()+&"-".to_string()+&day.to_string()+&" ".to_string()+&hour.to_string()
                +&":".to_string()+&min.to_string()+&":".to_string()+&sec.to_string();
                let dt = NaiveDateTime::parse_from_str(&result, "%Y-%m-%d %H:%M:%S");
                match dt {
                Ok(dt) => dt.to_string(),
                Err(e)=>e.to_string(),
            }}
        },
        "day" | "dy" | "dayofyear" | "d" | "dd" | "weekday" | "dw" | "w" =>{
            let dt = Utc.ymd(year.try_into().unwrap(), month.try_into().unwrap(), day.try_into().unwrap()) + Duration::days(input);
            let date = dt.to_string().replace("UTC","");
            return date;
        },
        "quarter" | "qq" | "q" => {
            if input >= 12{
            input = input * 3;
                input = month + input;
                let a = input % 12 ;
                let b = input / 12 ;
                month = a ;
                year = year + b ;
                let result = year.to_string()+&"-".to_string()+&month.to_string()+&"-".to_string()+&day.to_string()+&" ".to_string()+&hour.to_string()
                +&":".to_string()+&min.to_string()+&":".to_string()+&sec.to_string();
                let dt = NaiveDateTime::parse_from_str(&result, "%Y-%m-%d %H:%M:%S");
                match dt {
                    Ok(dt) => dt.to_string(),
                    Err(e)=>e.to_string(),
                }
            }
            else if input.to_string().contains('-'){
                input = input  * (-3);
                if month > input{
                    month = month - input;
                    let result = year.to_string()+&"-".to_string()+&month.to_string()+&"-".to_string()+&day.to_string()+&" ".to_string()+&hour.to_string()
                +&":".to_string()+&min.to_string()+&":".to_string()+&sec.to_string();
                let dt = NaiveDateTime::parse_from_str(&result, "%Y-%m-%d %H:%M:%S");
                match dt {
                    Ok(dt) => dt.to_string(),
                    Err(e)=>e.to_string(),
                }
                }
                else{
                let x = input / 12 ;  
                if x ==0 {year -= 1;
                month = month + 12;
                month = month - input;}
                else{ year = year - x;
                month = month + (12 *x);
                month = month - input;}
                let result = year.to_string()+&"-".to_string()+&month.to_string()+&"-".to_string()+&day.to_string()+&" ".to_string()+&hour.to_string()
                +&":".to_string()+&min.to_string()+&":".to_string()+&sec.to_string();
                let dt = NaiveDateTime::parse_from_str(&result, "%Y-%m-%d %H:%M:%S");
                match dt {
                    Ok(dt) => dt.to_string(),
                    Err(e)=>e.to_string(),
                }
                }
            }
            else{
                month = month + input;
                let result = year.to_string()+&"-".to_string()+&month.to_string()+&"-".to_string()+&day.to_string()+&" ".to_string()+&hour.to_string()
                +&":".to_string()+&min.to_string()+&":".to_string()+&sec.to_string();
                let dt = NaiveDateTime::parse_from_str(&result, "%Y-%m-%d %H:%M:%S");
                match dt {
                Ok(dt) => dt.to_string(),
                Err(e)=>e.to_string(),
            }
            }
        },
        "weeks" | "ww" | "wk" | "week"  => {
            let days = (input * 7) as i64;
            // let dt = Utc.ymd(year.try_into().unwrap(), month, day) + Duration::days(days);
            let dt = Utc.ymd(year.try_into().unwrap(), month.try_into().unwrap(), day.try_into().unwrap())
            .and_hms_milli(hour.try_into().unwrap(), min.try_into().unwrap(), sec.try_into().unwrap(),millisecs.try_into().unwrap()) + Duration::days(days);
            let result = dt.to_string().replace("UTC","");
            return result.trim().to_string();
        },
        "hour" | "hours" =>{
            let dt = Utc.ymd(year.try_into().unwrap(), month.try_into().unwrap(), day.try_into().unwrap())
                                .and_hms_milli(hour.try_into().unwrap(), min.try_into().unwrap(), sec.try_into().unwrap(),millisecs.try_into().unwrap()) + Duration::hours(input.into());
           let result = dt.to_string().replace("UTC","");
           return result.trim().to_string();
        }
        "minutes" | "mi" | "n" | "minute" =>{
            let dt = Utc.ymd(year.try_into().unwrap(), month.try_into().unwrap(), day.try_into().unwrap())
                                .and_hms_milli(hour.try_into().unwrap(), min.try_into().unwrap(), sec.try_into().unwrap(),millisecs.try_into().unwrap()) + Duration::minutes(input.into());
            let result = dt.to_string().replace("UTC","");
            return result.trim().to_string();
        },
        "seconds" | "sec" |"s" | "second"=> {
            let dt = Utc.ymd(year.try_into().unwrap(), month.try_into().unwrap(), day.try_into().unwrap())
                                .and_hms(hour.try_into().unwrap(), min.try_into().unwrap(), sec.try_into().unwrap()) + Duration::seconds(input.into());
            let result = dt.to_string().replace("UTC","");
            return result.trim().to_string();
        },
        "millisecond" | "ms" => {
            let dt = Utc.ymd(year.try_into().unwrap(), month.try_into().unwrap(), day.try_into().unwrap()).and_hms_milli(hour.try_into().unwrap(), min.try_into().unwrap(), sec.try_into().unwrap(),millisecs.try_into().unwrap()) 
                                 + Duration::milliseconds(input.into());
            let result = dt.to_string().replace("UTC","");
            return result.trim().to_string();
        },
        _ => todo!(),
    }
    
}

fn get_date_time(date: &str) -> (i64, i64, i64, i64, i64, i64, i64) {
    let mut year=0;
    let mut month=0;
    let mut day=0;
    let mut hour=0;
    let mut min=0;
    let mut sec=0;
    let mut millisecs =0;
    if date.len() == 10 || date.len() == 8 || date.len() == 9 {
    let _date_res = check_date(date);
    let v:Vec<&str> = date.split(&['-','/',' '][..]).collect();
    year = v[0].parse::<i64>().unwrap();
    month = v[1].parse::<i64>().unwrap();
    day = v[2].parse::<i64>().unwrap();}
    else if date.len() <= 19{
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
    let dt_fmt :Vec<&str> =date.split(&['.'][..]).collect();
    let _dd = check_date(dt_fmt[0]);
    if dt_fmt[1].parse::<i32>().unwrap() <= 999{
        let v:Vec<&str> = date.split(&['-','/',' ',':','.'][..]).collect();
    year = v[0].parse::<i64>().unwrap();
    month = v[1].parse::<i64>().unwrap();
    day = v[2].parse::<i64>().unwrap();
    hour = v[3].parse::<i64>().unwrap();
    min = v[4].parse::<i64>().unwrap();
    sec = v[5].parse::<i64>().unwrap();
    millisecs = v[6].parse::<i64>().unwrap();
    }}
    (year, month, day, hour, min, sec, millisecs)
}

fn check_date(date_only:&str)->Result<(),ParseError>{
    if date_only.len() == 10 || date_only.len() == 9 || date_only.len() == 8{
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
}