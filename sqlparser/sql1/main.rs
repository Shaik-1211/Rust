//Current working file
mod option;
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use sqlparser::ast::{SetExpr, Statement};
extern crate csv;
mod lib;
use lib::{get_column_name,get_column_data,get_name};

#[allow(unused_variables)]
fn main() {
    let dialect=GenericDialect{};
    let sql= "SELECT firstname,lastname,concat(firstname,lastname) 
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
//    {
//       println!("{:?}",i.name);
//    }
//    if let Some(where_eq) = select.selection.clone() {
//     let expression = where_fn(where_eq);
//     println!("{}", expression);
    // if let Err(e)=concat("data.csv"){
    //     println!("{:?}",e);
    // }
    let column_data=get_column_data(columns,"data.csv");
    
}

