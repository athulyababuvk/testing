fn main(){
    println!("Hello world !");
    addfn(34,92);
}

//function for adding two numbers 
pub fn addfn(a:i32, b:i32){
    let c = a + b ;
    println!("Sum of `a` and `b` is {}",c);
}

