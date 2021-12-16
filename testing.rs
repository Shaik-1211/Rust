#[derive(Debug,Copy,Clone)]
struct Rectangle{
    length:u32,
    width:u32,
}

impl Rectangle{
fn can_hold(&self,other:Rectangle)->bool{
    self.length > other.length && self.width > other.width
}
}

fn limit(a:i32,b:i32)->i32
{
    let c=a+b;
    if c>100{
        panic!("The sum of two number must be less than 100 or greater than 1");
    }
    else if c<1
    {panic!("The sum of two number must be less than 100 or greater than 1");}
    else
    {c}
}

fn division(a:i32,b:i32)->i32{
    if b==0
    {panic!("Denominator should not be zero");}
    else 
    {a/b}
}

fn product(a:i32,b:i32)->i32{
    a*b
}
fn main()
{
    println!("Hi ");
    let larger=Rectangle{length:7,width:10};
    let smaller=Rectangle{length:8,width:2};
}

#[cfg(test)]
mod testing
{   use super::*;
    #[test]
    fn check_hold(){
        let larger = Rectangle{length:13,width:10};
        let smaller = Rectangle{length:11,width:5};
        //assert! macro will does nothing if the condition provided evaluates 
        // to true otherwise if the condition provided evaluates to false the 
        // it will calls the panic macro which causes test to fail 
        assert!(!smaller.can_hold(larger));
    }

    #[test]
    // Here should_panic attribute ensures that the test should panic 
    #[should_panic(expected="The sum of two number must be less than 100 or greater than 1")]
    fn limit_checker(){
        assert_eq!(113,limit(101,12));
    }

    #[test]
    fn equals()
    {
        // assert_eq! macro  will ensure that the result provided in assert_eq 
        // and the result yielded by the function are equal.
        assert_eq!(40,product(5,8));
    }

    #[test]
    fn not_equals()
    {   //assert_ne! macro will check that the provided value and the value 
        // obtained by the function are not equal
        assert_ne!(10,division(19,5))
    }
}