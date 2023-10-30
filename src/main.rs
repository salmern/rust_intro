
mod variable;
fn main() {

    let x = -34; //immutable 
    let _f = 5.6; 
    //let m: usize = 50; 
 
   let tuple = (4,5); 
 
   let a = [1,2,3,4]; 
   let mut v = vec![1,2,3,4,200]; 
   v.pop(); 
   v.push(56); 
 
   let s = &v[0..3]; 
   
     println!("Hello, world!, {}", x );
     println!("{:?}", tuple); 
     println!("{:?}", v); 
     println!("{:?}", s); 
 
 
   enum Direction{
     Up, 
     Down, 
   }
 
   let down = Direction::Down; 
 
   match down {
     Direction::Down => println!("down"), 
     Direction::Up => println!("up"), 
   }
 
   #[derive(Debug)]
   struct Person {
     name: String, 
     age: i32, 
   }
 
   impl Person {
   
 
     fn is_adult(&self) -> bool {
       if self.age >= 18 {
         true 
       } else {
         false 
       }
     }
 
     fn new(name: String) -> Person {
       Person{
         name: name, 
         age: 18
       }
     }
   }
 
     let p1 = Person{
     name: String::from("binkabir"), 
     age: 39
   }; 
 
     let p2 = Person::new("idris".to_string()); 
       
    println!("{:?}", p1); 
    println!("{:?}", p2); 
    println!("{:?}", p2.is_adult()); 
 
   let c = |x| x + 6; 
 
   let t: Vec<i32> = a.iter().map(c).collect(); 
   println!("{:?}", t); 
   
   
 }