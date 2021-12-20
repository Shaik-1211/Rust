struct Counter {
    num:u32,
}
impl Counter{
    fn new()->Counter{
        Counter{num:0}
    }
}
impl Iterator for Counter{
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item>{
        if self.num<5{
            self.num+=1;
            Some(self.num)
        }
        else{None}
    }
}
fn main(){
    let numbers=Counter::new();
    for i in numbers {
        println!("{:?}",i);
    }
}
