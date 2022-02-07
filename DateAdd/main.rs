use crate::add_date_time::add_date_time::add;

mod add_date_time;

fn main() {
    let new_date = add("quarter",- 3,"2022-02-23 12:2:33");
    println!("{:?}",new_date);
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_year(){
        let result = add("year",87,"2021-02-04");
        let expected = "2108-02-04 00:00:00".to_owned();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_month(){
        let result = add("month",77,"2021-02-04");
        let expected = "2027-07-04 00:00:00".to_owned();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_day() {
        let result = add("day",200,"2021-02-04");
        let expected = "2021-08-23".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_week(){
        let result = add("week", 5,"2000-01-01");
        let expected = "2000-02-05 00:00:00".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_quarter(){
        let result = add("quarter",-8,"2017-11-16");
        let expected = "2015-11-16 00:00:00".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_dayofyear(){
        let result = add("dayofyear",70,"2020-11-30");
        let expected = "2021-02-08".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_weekday(){
        let result = add("weekday",-23,"1937-08-17");
        let expected = "1937-07-25".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_hour(){
        let result = add("hour",34,"1988-05-20");
        let expected = "1988-05-21 10:00:00".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_minute(){
        let result = add("minute",646,"1900-9-12 17:54:45");
        let expected = "1900-09-13 04:40:45".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_seconds(){
        let result = add("second",-200,"1923-07-22");
        let expected = "1923-07-21 23:56:40".to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_millisec(){
        let result = add("ms",-298,"1946-08-30 18:55:49");
        let expected = "1946-08-30 18:55:48.702".to_string();
        assert_eq!(result, expected);
    }
}


