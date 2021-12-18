use std::io;
fn main()
{
    let mut var1=String::new();
    io::stdin().read_line(&mut var1).expect("Error occured");
    let mut var1=var1.trim().parse();

    let mut var2=String::new();
    io::stdin().read_line(&mut var2).expect("Error occured");
    let mut var2=var2.trim().parse();

    println!("1.Add 2.Difference 3.Product 4.Division");
    let mut option=String::new();
    io::stdin().read_line(&mut option).expect("Error occured");
    let mut option=option.trim().parse();

    //Closures are used here
    let sum=|num1,num2|{num1+num2};
    let diff=|num1,num2|{num1-num2};
    let product=|num1,num2|{num1*num2};
    let division=|num1,num2|{num1/num2};

    //Match the option selected and get the output from the respective closure
    match option{
        1=>println!("{:?}",sum(var1,var2)),
        2=>println!("{:?}",diff(var1,var2)),
        3=>println!("{:?}",product(var1,var2)),
        4=>{if var2!=0{
            println!("{:?}",division(var1,var2));}
            else {
                println!("num2 should not be equal to zero");
            }}
    }


}