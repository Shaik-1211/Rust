use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use sqlparser::ast::{Expr, ObjectName, Select, SelectItem, SetExpr, Statement, TableFactor};
extern crate csv;
use std::error::Error;

#[derive(Debug)]
struct ColumnInfo {
        name: String,
        alias: Option<String>,
        func: Option<String>,
}
impl ColumnInfo {
    fn new() -> ColumnInfo {
            ColumnInfo {
                name: "".to_string(),
                alias: None,
                func: None,
            }
        }
    }


fn concat(path: &str) -> Result<(),Box<dyn Error>> {
    let mut rdr=csv::Reader::from_path(path)?;
    let headers=rdr.headers()?;
    println!("{:?} {:?} full_name",&headers[0],&headers[1]);//headers
    for result in rdr.records() {
        let record=result?;
    let mut full_name=record[0].to_string();
        full_name.push('.');
        let mut var=String::new();
        let (first,last)=full_name.split_at(1);//splitting the first name at index 1 
        var.push_str(first);
        full_name.push_str(var.as_str());//Adding the first letter of the full name
        println!("{:?} {:?} {:?}",&record[0],&record[1],full_name);//printing the expected output 
    }
    Ok(())
}


fn main() {
    let dialect=GenericDialect{};
    let sql= "SELECT firstname,lastname,concat(full_name,last_name) 
              FROM data";
    let mut ast = Parser::parse_sql(&dialect, sql).unwrap();
    let query2 = match ast.pop().unwrap() {
        Statement::Query(query2) => query2,
        _ => return println!("Not a select query"),
    };
//    println!("{:?}", query2);

   let select =match  query2.body{
       SetExpr::Select(select)=>select,
       _=>return println!("Not a select query"),
   };
//    println!("{:?}", select);
   let name = get_name(select.clone());
   println!("Name of the table is {:?}", name);
   let columns = get_column_name(select.projection.clone());
//    for i in &columns
   {
    //   println!("{:?}",i.name);
   }
//    if let Some(where_eq) = select.selection.clone() {
//     let expression = where_fn(where_eq);
//     println!("{}", expression);
    // if let Err(e)=concat("data.csv"){
    //     println!("{:?}",e);
    // }
    if let Err(e)=get_column_data(columns,"data.csv"){
        println!("{:?}",e);
    }
}

fn get_column_data(input:Vec<ColumnInfo>,file:&str)->Result<(),Box<dyn Error>>{
    let mut rdr=csv::Reader::from_path(file)?;
    let mut rdr2=csv::Reader::from_path(file)?;
    let headers=rdr2.headers()?;
    for result in rdr.records() {
        let record =result?;
        for i in &input{
            if i.name == headers[0].to_string(){
                println!("{:?}",&record[0]);
                // break;
            }
            else if i.name == headers[1].to_string(){
                println!("{:?}",&record[1]);
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
    Ok(())
    
}
fn get_name(select:Box<Select>)->String {
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

//A funcction to get column names.
fn get_column_name(projection: Vec<SelectItem>) -> Vec<ColumnInfo> {
    let mut columns = Vec::new();
    let mut column = ColumnInfo::new();
    let mut func: Option<String> = None;
    let alias: Option<String> = None;
    let mut name: String;
    for item in projection {
        match item {
            SelectItem::UnnamedExpr(item) => {
                match &item {
                    Expr::Function(f) => {
                        println!("function : {}", f.name.to_string());
                        func = Some(f.name.to_string());
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

