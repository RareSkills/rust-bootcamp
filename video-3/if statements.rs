fn main() {
    let x  = 44;
    
    let msg = if x == 42 {
        "the meaning of life and all existence"
    }
    else if x % 2 == 0 {
        "even"
    }
    else {
        "odd"
    };
    
    println!("x is {}", msg);
    
    /*
    let x  = 42;
    
    if (x == 42) {
        println!("x is the meaning of life \ and all existence");
    }
    else if (x % 2 == 0) {
        println!("x is even");
    }
    else {
        println!{"x is odd"};
    }
    */
}