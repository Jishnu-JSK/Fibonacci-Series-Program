use std::io;
fn fibo()
{
    let mut n=String::new();
    println!("Enter a number:");
    io::stdin().read_line(&mut n).unwrap();
    let number:i32=n.trim().parse().expect("Failed to read!!");
    n.clear();
    
    let mut x:i32=0;
    let mut y:i32=1;
    let mut i:i32=0;
    
    println!("Fibonacci Series");
    while i < number {
            print!("{} ",x);
            let z=x+y;
            x=y;
            y=z;
            i+=1;
        }
    println!();
}
fn main()
{
    fibo()
}