struct Sq {
  u: u8,
  v: u8,
  
  hidden: bool, //should be true by default
  mine: bool,
  
  nearby: u8
}

fn main() {
  let example_mine_sq_u1v1 = Sq{
    u       = 1, 
    v       =1,
    
    hidden  = true,
    mine    = true,
    
    nearby  = 0 //should be set with a function, after the whole map is generated.
  }
}
