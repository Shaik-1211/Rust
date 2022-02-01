pub(crate) mod check_date{
    pub fn isdate(date:&str) -> bool {
        if date.len()==4 {
             return true;}
        else{
        let v:Vec<&str> = date.split(&['-','/',' '][..]).collect();
        let year=v[0].parse::<i32>().unwrap();
        let month=v[1].parse::<i32>().unwrap();
        let date=v[2].parse::<i32>().unwrap();
        if v[0].len() == 4 && ( v[1].len() == 2 ||v[1].len() ==1 ) && ( v[2].len() ==2 ||v[2].len() ==1 ){
            if month <=12 {
                if month == 2{
                   let leap = crate::isdate::leapyear(year,date);
                    if leap==true{return true;}
                    else{return false;}
                }
                }
                else if date ==31{
                    match month{
                        1=>{return true;},
                        3=>{return true;},
                        5=>{return true;},
                        7=>{return true;},
                        8=>{return true;},
                        10=>{return true;},
                        12=>{return true;},
                        _=>{return false;},
                    }}
                    else if date <= 30{
                        match month{
                            1=>{return true;},
                            3=>{return true;},
                            4=>{return true;},
                            5=>{return true;},
                            6=>{return true;},
                            7=>{return true;},
                            8=>{return true},
                            9=>{return true;},
                            10=>{return true;},
                            11=>{return true;},
                            12=>{return true;},
                            _=>{return false},
                        }}
                    else {return false;}
            }
            else {return false;}
        }
        else{return false;}
    }
    }
}

pub fn leapyear(x:i32,z:i32) -> bool {
                   if z <= 28{return true;}
                   else if z <= 29 {
                        if x % 100 == 0
                        { if x % 400 ==0{
                        return true;}
                        else{return false;}}
                        else {
                        return false;}
                        }
                    else if x % 4 == 0 {
                    return true;}
                    else{return false;}
}