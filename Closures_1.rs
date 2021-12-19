use std::io;
fn main()
{   println!("Enter an integer");
    let mut var1=String::new();
    io::stdin().read_line(&mut var1).expect("Error occured");
    let mut var1=var1.trim().parse().expect("Enter a valid input");

    println!("Enter another integer");
    let mut var2=String::new();
    io::stdin().read_line(&mut var2).expect("Error occured");
    let mut var2=var2.trim().parse().expect("Enter a valid input");

    println!("1.Add 2.Difference 3.Product 4.Division");
    let mut option=String::new();
    io::stdin().read_line(&mut option).expect("Error occured");
    let mut option=option.trim().parse().expect("Enter a valid input");

    //Closures are used here
    let sum=|num1,num2|{num1+num2};
    let diff=|num1,num2|{num1-num2};
    let product=|num1,num2|{num1*num2};
    let division=|num1,num2|{num1/num2};

    //Match the option selected and get the output from the respective closure
    match option{
        1=>println!("The sum of given two integers is {:?}",sum(var1,var2)),
        2=>println!("The difference of given two integers is {:?}",diff(var1,var2)),
        3=>println!("The Product of given two integers is {:?}",product(var1,var2)),
        4=>{if var2!=0{
            println!("The remainder when first integer is divided by second integer is {:?}",division(var1,var2));}
            else {
                println!("num2 should not be equal to zero");
            }}
    }


}
