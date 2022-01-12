//A test to check the output of the column_data(the columns specified in the query).
#[cfg(test)]
    mod tests {
        use crate::*;
        #[test]
        fn test_get_column_data(){
            let dialect=GenericDialect{};
    let sql= "SELECT firstname,lastname,concat(full_name,last_name) 
              FROM data";
    let mut ast = Parser::parse_sql(&dialect, sql).unwrap();
    let query2 = match ast.pop().unwrap() {
        Statement::Query(query2) => query2,
        _ => return println!("Not a select query"),
    };
   let select =match  query2.body{
       SetExpr::Select(select)=>select,
       _=>return println!("Not a select query"),
   };
   let columns = crate::get_column_name(select.projection.clone());
            let column_data = crate::get_column_data(columns,"data.csv");
            let mut result =Vec::new();
             for i in 0..column_data.len() {
                result.push(column_data[i].as_str());
            };
            let expected=vec!["Salma","shaik","khalid","shaik","John","Doe","Taaha",
            "khan","Uzair","khan","Mujeeb","shaik","Waseem","shaik","Firoz","khan","Huzaif","khan"];
            assert_eq!(result,expected);
        }

        //A test to check the output of the function in the query.
    #[test]
    fn test_concat_function(){
        let mut input_vec=Vec::new();
        input_vec.push("firstname".to_string());
        input_vec.push("lastname".to_string());
        let output=sql3::concat(input_vec);
        let mut result=Vec::new();
        for i in 0..output.len(){
            result.push(output[i].as_str());
        }
        let expected=vec!["Salma.S","khalid.k","John.J","Taaha.T","Uzair.U",
        "Mujeeb.M","Waseem.W","Firoz.F","Huzaif.H"];
        assert_eq!(result, expected);
    }

    }
