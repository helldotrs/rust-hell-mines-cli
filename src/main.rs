struct Sq {
  u: u8,
  v: u8,
  
  hidden: bool, //should be true by default
  mine: bool,
  
  nearby: u8 //how to mut??
              //shaddow the original struct?
}

fn main() {
  
  let rows = 2;
  let cols = 2;
  
  let mut my_grid = vec![
    vec![
      MyStruct { 
        u: 0, v: 0, //FIXME: auto increase
        hidden: true, mine: true, //FIXME: set mine with auto function
        nearby: 0, //FIXME/ADD: changed afterwards, based on nearby
      }; cols
    ]; rows
  ];

  
  
  
  let example_mine_sq_u1v1 = Sq{
    u       = 1, 
    v       = 1,
    
    hidden  = true,
    mine    = true,
    
    nearby  = 0 //should be set with a function, after the whole map is generated.
  }
}

fn check_nearby(a) --> u8 {
  //FIXME
}
