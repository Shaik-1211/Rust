// use sqlparser::dialect::GenericDialect;
// use sqlparser::parser::Parser;
use sqlparser::ast::{Expr, ObjectName, Select, SelectItem,TableFactor};
extern crate csv;



#[derive(Debug)]
pub struct ColumnInfo {
        pub name: String,
        pub alias: Option<String>,
        pub func: Option<String>,
}
impl ColumnInfo {
    pub fn new() -> ColumnInfo {
            ColumnInfo {
                name: "".to_string(),
                alias: None,
                func: None,
            }
        }
    }

pub fn concat(input:Vec<String>){
    let mut rdr=csv::Reader::from_path("data.csv").expect("failed to read the file");
    let headers=rdr.headers().expect("failed to read the headers");
    let mut matched=false;
    for i in 0..input.len(){
        for j in 0..3{
        if input[i]==&headers[j]{
            matched=true;
        }
        }
    }
    if matched==true{
        for result in rdr.records() {
            let record=result.expect("failed to get the record");
        let mut full_name=record[0].to_string();
            full_name.push('.');
            let mut var=String::new();
            let (first,_last)=full_name.split_at(1);//splitting the first name at index 1 
            var.push_str(first);
            full_name.push_str(var.as_str());//Adding the first letter of the full name
            println!("{:?} {:?} {:?}",&record[0],&record[1],full_name);//printing the expected output 
        }
    }
}

pub fn get_column_data(input:Vec<ColumnInfo>,file:&str)->Vec<String>{
    let mut rdr=csv::Reader::from_path(file).expect("failed to read file");
    let mut rdr2=csv::Reader::from_path(file).expect("failed to read file");
    let mut result_vector=Vec::new();
    let mut vector1=Vec::new();
    let mut vector2:Vec<String>=Vec::new();
    let headers=rdr2.headers().expect("failed to read headers");
    for result in rdr.records() {
        let record =result.expect("failed to get the record");
        for i in &input{
            if i.name == headers[0].to_string(){
                // println!("{:?}",&record[0]);
                vector1.push(record[0].to_string());
                // break;
            }
            else if i.name == headers[1].to_string(){
                // println!("{:?}",&record[1]);
                vector2.push(record[1].to_string());
                break;
            }
            else if i.name == headers[2].to_string(){
                println!("{:?}",&record[2]);
                break;
            }
            else {println!("NO column present with this name");
            break;}
        }
        // break;
    }
    println!("Columns in the query are : ");
    for  i in 0..=vector2.len()-1{
    println!("{:?} {:?}",vector1[i],vector2[i]);
    result_vector.push(vector1[i].to_string());
    result_vector.push(vector2[i].to_string());}
    result_vector    
}    


pub fn get_name(select:Box<Select>)->String {
    let rel = &select.from[0];
    match &rel.relation{
        TableFactor::Table{name,..}=>match name{
            ObjectName(t)=>{
                let table_name = &t[0].value;
                return table_name.to_string();
            },
        }
        _ =>return format!("Not implemented !"),
        
    }
}

pub fn get_args(arg:Vec<sqlparser::ast::FunctionArg>) ->Vec<String> {
    // println!("argument {:?}",arg);
    let mut par_vec = Vec::new();
    let mut par1=String::from("hi");
    for i in 0..arg.len() {
    let arg1 = &arg[i];
    match arg1{
        sqlparser::ast::FunctionArg::Named {..}=>println!("not a named argument"),
        sqlparser::ast::FunctionArg::Unnamed(data)=>match data{
            Expr::Identifier(name)=>par1=name.value.clone(),
            _=>println!("not a identifier"),
        }
    }
    par_vec.push(par1.clone());}
    // for i in 0..par_vec.len(){
    // print!("{:?}",par_vec[i]);}
    // print!("concated");
    // println!("");
    par_vec
    
}

#[allow(unused_assignments)]
//A funcction to get column names.
pub fn get_column_name(projection: Vec<SelectItem>) -> Vec<ColumnInfo> {
    let mut columns = Vec::new();
    let mut argsvec = Vec::new();
    let mut column = ColumnInfo::new();
    let mut func: Option<String> = None;
    let _alias: Option<String> = None;
    let mut name: String;
    for item in projection {
        match item {
            SelectItem::UnnamedExpr(item) => {
                match &item {
                    Expr::Function(f) => {
                        println!("function : {}", f.name.to_string());
                        func = Some(f.name.to_string());
                        for i in 0..=f.args.len()-1{
                        argsvec.push(f.args[i].clone());
                        }
                        if f.name.to_string().to_lowercase() =="concat".to_string(){
                            let result =get_args(argsvec.clone());
                            println!("THe output of the concat function in the query : ");
                            // println!("result is {:?}",result);
                            let _columns=concat(result);
                            // println!("Result is {:?}",result);
                        }
                        name = item.to_string();
                    }
                    _ => {
                        name = item.to_string();
                        func = None
                    }
                };
                column = ColumnInfo {
                    name,
                    alias: None,
                    func: func,
                };
            }
            SelectItem::ExprWithAlias { expr, alias } => {
                match expr {
                    Expr::Function(f) => {
                        func = Some(f.name.to_string());
                        name = f.args[0].to_string();
                    }
                    _ => {
                        name = expr.to_string();
                        func = None;
                    }
                };
                column = ColumnInfo {
                    name,
                    alias: Some(alias.to_string()),
                    func,
                };
            }
            SelectItem::Wildcard => {
                column = ColumnInfo {
                    name: "*".to_string(),
                    alias: None,
                    func: None,
                }
            }
            _ => {
                column = ColumnInfo {
                    name: "error".to_string(),
                    alias: None,
                    func: None,
                }
            }
        };
        columns.push(column);
    }
    columns
}

