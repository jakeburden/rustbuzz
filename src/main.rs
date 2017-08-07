fn main() {
  for i in 1..101 {
    if i % 3 == 0 && i % 5 ==0 {
      println!("{} fizbuzz", i) 
    }

    else if i % 3 == 0 {
      println!("{} fizz", i)
    }
    
    else if i % 5 == 0 {
      println!("{} buzz", i) 
    }
    
    else { println!("{}", i) }
  }
}
