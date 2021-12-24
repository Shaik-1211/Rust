//Find the factorial of the given number
fn factorial(a:i32)->i32{
    if a==0 || a==1
    {1}
    else
    {
        a * factorial(a-1)
    }
}

fn main() {
    let num=4;
    println!("{:?}",factorial(num));}