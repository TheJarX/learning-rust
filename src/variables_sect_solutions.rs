fn main(){
    // Farenheit degrees
    let deg = 127;
    println!("{} Farenheit degrees are {} Celcius degree.", deg, faren_to_celc(deg));
    // The 10th Fibo number
    let position = 10;
    println!("Fibonacci num for position {} is : {}", position,fibonacci(position));
    // And a simple repetitive text...
    twelve_days();
    
}

fn faren_to_celc(tem: i32) -> i32 { 
    (tem - 32) * 5/9 
}

fn fibonacci(mut nth: i32) -> i32 {
  let mut n2: i32 = 0;
  let mut n1: i32 = 1;
  let mut n: i32 = 0;
  
  if nth <= 2 { return if nth - 1 < 0 { 0 } else { nth - 1 }; }
  
  while nth-2 != 0 { 
    n = n1 + n2;
    n2 = n1;
    n1 = n;
    nth -= 1;
  }
  
  return n;
}

fn twelve_days() {
    for i in 1..13 {
        println!("On the {} day of Christmas my true love gave to me", i);
    }
}
