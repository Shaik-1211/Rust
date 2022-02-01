// use crate::isdate::check_date;

mod isdate;
fn main() {
    let dateadded = dateadd("millisec",4000,"2001-6-30");
    println!("{}",dateadded);
}

#[allow(unused_mut)]
fn dateadd(category: &str,input :i32,date:&str)->String{
   let sign=sign(input);
   let v:Vec<&str> = date.split(&['/','-'][..]).collect();
   let mut year=v[0].parse::<i32>().unwrap();
   let mut month=v[1].parse::<i32>().unwrap();
   let mut day=v[2].parse::<i32>().unwrap();
   let mut hour_format = "00:00:00.000";
   let hv:Vec<&str> = hour_format.split(&[':','.'][..]).collect();
   let mut hour =hv[0].parse::<i32>().unwrap();
   let mut min = hv[1].parse::<i32>().unwrap();
   let mut sec = hv[2].parse::<i32>().unwrap();
   let mut millisec = hv[3].parse::<i32>().unwrap();
    if sign==1{
        match category{ 
            "year" => {
                if input<=9000{
                    year+=input;
                    println!("{:?}", year);
                }
                else {
                    println!("Try with a smaller year");}
            },
            "month" =>{
                if month <=12{
                    month += input;
                    if month > 12{
                        let m1= month % 12;
                        let m2= month / 12;
                        month = m1;
                        year = year + m2;
                    }
                }
                else {
                    let a = month % 12;
                    let b = month / 12;
                    month=a;
                    year = year + b;
                }
            },
            "day"=>{
                day = day+input;
                let day_result = add_day(month, day, year);
                day = day_result.0;
                month = day_result.1;
            },
            "week"=>{
                let week_days = input * 7;
                day=day + week_days;
                let week_result = add_day(month, day, year);
                day = week_result.0;
                month = week_result.1;
            },
            "quarter"=> {
                let q = input * 3;
                month = month +q ;
            },
            "hour"=>{
                if input >= 24{
                    let h1 = input % 24 ;
                    let h2 = input / 24 ;
                    day = day + h2 ;
                    hour = h1 ;
                    let result = add_day(month, day, year);
                    day = result.0;
                    month = result.1;
                    if month > 12{
                        let m1 = month % 12 ;
                        let m2 = month / 12;
                        month = m1 ;
                        year = year + m2;}
                }
                else {
                    hour = input ;
                }
            },
            "min"=>{
                if input > 60 {
                     let mi1 = input % 60;
                     let mi2 = input / 60;
                     hour = hour + mi2;
                     min = mi1;
                }
                else{
                    min = input ;
                }
            },
            "sec"=> {
                if input > 60{
                    let s1 = input % 60;
                    let s2 = input / 60;
                    min = min + s2;
                    sec = s1;
                }
                else{
                    sec = input;
                }
            },
            "millisec"=>{
                if input >1000{
                    let ms1 = input % 1000;
                    let ms2 = input / 1000;
                    sec = sec + ms2;
                    millisec = ms1;
                }
                else{
                    millisec = input;
                }
            },
            _=>todo!(),
        }
    }
   

   match category{
       "hour" => {
           let output = year.to_string()+&"-".to_string()+&month.to_string()+&"-".to_string()+&day.to_string()
                                +&"  ".to_string()+&hour.to_string()+&":".to_string()+&min.to_string()+&":".to_string()
                                +&sec.to_string()+&":".to_string()+&millisec.to_string();
                                output
       },
       "min" => {
        let output = year.to_string()+&"-".to_string()+&month.to_string()+&"-".to_string()+&day.to_string()
                            +&"  ".to_string()+&hour.to_string()+&":".to_string()+&min.to_string()+&":".to_string()
                            +&sec.to_string()+&":".to_string()+&millisec.to_string();
                            output
       },
       "sec" => {
        let output = year.to_string()+&"-".to_string()+&month.to_string()+&"-".to_string()+&day.to_string()
                            +&"  ".to_string()+&hour.to_string()+&":".to_string()+&min.to_string()+&":".to_string()
                            +&sec.to_string()+&":".to_string()+&millisec.to_string();
                            output
       },
       "millisec" => {
        let output = year.to_string()+&"-".to_string()+&month.to_string()+&"-".to_string()+&day.to_string()
                            +&"  ".to_string()+&hour.to_string()+&":".to_string()+&min.to_string()+&":".to_string()
                            +&sec.to_string()+&":".to_string()+&millisec.to_string();
                            output
       },
       _=>{
        if year >= 10000{return "not a valid input".to_string()}
        else{
            if month > 12{
                let m1 = month % 12 ;
                let m2 = month / 12;
                month = m1 ;
                year = year + m2;
            }
        let output_date=year.to_string()+&"-".to_string()+&month.to_string()+&"-".to_string()+&day.to_string();
        output_date}
       },
    //    _=>todo!(),
   }
  
}


fn sign(num:i32)-> i32{
    let a: i32 = num;
    // let size = a.len();
   let b = a.to_string();
   let mut output=1;
   let c:Vec<char>=b.chars().collect();
   if b.len()==1{
       for i in &c{
           if *i =='0'{
               output =0;
           }
       }
   }
   for i in c{
        if i == '-'{
                    output =-1;
                    }
                   
       else { output=1;
    }
   }
   output
}

#[allow(unused_assignments)]
fn add_day(mut month: i32,mut day: i32,year: i32)->(i32, i32) {
    match month{
        1 =>{let d1 = day % 31;
            let d2 = day / 31;
            month = month + d1;
            day = d2;},
        2=>{let leap = isdate::leapyear(year.into(),day);
            if leap == true{
                if day > 29{
                let f1=day%29;
                let f2 =day / 29;
                month = month + f2;
                day = f1;   }
                else{
                    day = day ;
                    month = month;
                }}
            else{
                if day > 28{
                let f1 = day %28;
                let f2 = day / 28;
                month = month + f2;
                day = f1; }
                else{
                    day = day ;
                    month = month ;
                }
            }},
            3=>{
                let d1 = day % 31;
            let d2 = day / 31;
            month = month + d1;
            day = d2;
            },
            4=>{
                let d1 = day % 30;
            let d2 = day / 30;
            month = month + d1;
            day = d2;
            },
            5=>{
                let d1 = day % 31;
            let d2 = day / 31;
            month = month + d1;
            day = d2;
            },
            6=>{
                let d1 = day % 30;
            let d2 = day / 30;
            month = month + d1;
            day = d2;
            },
            7=>{
                let d1 = day % 31;
            let d2 = day / 31;
            month = month + d1;
            day = d2;
            },
            8=>{
                let d1 = day % 31;
            let d2 = day / 31;
            month = month + d1;
            day = d2;
            },
            9=>{
                let d1 = day % 30;
            let d2 = day / 30;
            month = month + d1;
            day = d2;
            },
            10=>{let d1 = day % 31;
                let d2 = day / 31;
                month = month + d1;
                day = d2;},
            11=>{let d1 = day % 30;
                let d2 = day / 30;
                month = month + d1;
                day = d2;},
            12=>{let d1 = day % 31;
                let d2 = day / 31;
                month = month + d1;
                day = d2;},
            _=>println!("not a valid month"),
    }
    (day,month)
}