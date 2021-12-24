 //A program to check how many times each character has been repeated in the given expression.
use std::collections::HashMap;
 fn main() {  
 let input="kkjienxcv64$#u*j*n$sn545oj325454";
        let _exp=input.chars();
        let mut expvec=Vec::new();
        let mut exphashmap=HashMap::new();
        for i in input.chars(){
            expvec.push(i);
        }

        println!("{:?}", expvec);
        for i in expvec.iter() {
            let mut count=0;
            for j in expvec.iter() {
                if i == j
                {count+=1;}
            }
            exphashmap.insert(i,count);
        }
        for i in exphashmap.iter() {
            print!("{:?}",i);
        }
    }