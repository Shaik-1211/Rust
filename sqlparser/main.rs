use sqlparser::dialect::GenericDialect;
    use sqlparser::parser::Parser;
    use sqlparser::ast::{Expr, ObjectName, Select, SelectItem, SetExpr, Statement, TableFactor};

#[derive(Debug)]
struct columnInfo {
        name: String,
        alias: Option<String>,
        func: Option<String>,
}
impl columnInfo {
    fn new() -> columnInfo {
            columnInfo {
                name: "".to_string(),
                alias: None,
                func: None,
            }
        }
    }
    fn main() {
    let dialect = GenericDialect {}; 
    
    let sql = "SELECT a,b \
    FROM table_1 \
    WHERE a > b AND b < 100 \
    ORDER BY a DESC, b";
    
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
   println!("{:?}", select);
   let name = get_name(select.clone());
   println!("Name of the table is {:?}", name);
   let columns = get_columns(select.projection.clone());
   println!("{:?}",columns);
   
}

//A funcction to get table name.
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

//A funcction to get columnumns.
fn get_columns(projection: Vec<SelectItem>) -> Vec<columnInfo> {
    let mut columns = Vec::new();
    let mut column = columnInfo::new();
    let mut func: Option<String> = None;
    let mut alias: Option<String> = None;
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
                column = columnInfo {
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
                column = columnInfo {
                    name,
                    alias: Some(alias.to_string()),
                    func,
                };
            }
            SelectItem::Wildcard => {
                column = columnInfo {
                    name: "*".to_string(),
                    alias: None,
                    func: None,
                }
            }
            _ => {
                column = columnInfo {
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
