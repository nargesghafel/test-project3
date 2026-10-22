use std::io;
fn main() {
    let mut number =String::new();
    io::stdin().read_line(&mut number).expect("Failed to read line");
    //let number: i32 = number.trim().parse().expect("Please type a number!");
    let number: i32 = match number.trim().parse() 
        {
        Ok(number) if number >0 => number,
        _ => {
           print!("you must enter a number");
           return;
            }
        };
    
    let mut output = String::new();
    for _ in 0..number {
         let mut x =String::new();
         io::stdin().read_line(&mut x).expect("Failed to read line");
        //let mut x: i32 = x.trim().parse().expect("Please type a number!");
        let mut x: u64 = match x.trim().parse() 
        {
        Ok(x) if x >0 => x,
        _ => {
           print!("you must enter a number");
           return;
            }
        };
        let mut count = 1;
        while x > 1{
            if x%2==0{
                x = x/2;
                count +=1;

            }
            else{
                x = 3*x+1;
                count +=1;
            }
           
        } output.push_str(&format!("{}\n", count));
    
    }print!("{}", output);

}



