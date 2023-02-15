fn main() {
    // variable and mutability
    let x = 5;
    // need to define let  else its generate an error
    let x = x + 1;
    // user defined inner socpe
    {
        let x = x * 2;
        println!("The value of inner scope is {}", x)
    }
    println!("The vlaue of x is : {x}");
    // this is string type
    let space = "  ";
    let spaces = space.len();
    // this is number type.
    println!("The length of space is {}", spaces);
    
    // Data types
    let guess: u32 = "32".parse().expect("Not a number, It can be a char or string with ");
    println!("Guess number is {}", guess);
    
    /*
        scalar type represents a single value. Rust has 4 scalar typessss:
        1. integer,ting-point 3.numbers 4.Booleans and characters.
        
        Each signed variant can store numbers from -(2n - 1) to 2n - 1 - 1
        inclusive, where n is the number of bits that variant uses. 
        So an i8 can store numbers from -(27) to 27 - 1, 
        which equals -128 to 127. Unsigned variants can store numbers from 0 to 2n - 1, 
        so a u8 can store numbers from 0 to 28 - 1, which equals 0 to 255.
    */
    
    // number literals also use _ visual seperaor
    
    let num  =  1_0000_5;
    println!("num is {}", num);
    // f64
    let _f_val = 3.0;
    // f32
    let _f2_val : f32 = 3.0;
    
    // numaric opeation
    
    // addition
    
    let sum = 5 + 10;
    
    // subtraction
    let difference = 10.9 -5.9;
    
    // floored value 
    let floor_value = 3 / 2 ;
    
    // remainder 
    
    let remainder  = 43 % 5;
    println!("Summation is {sum} difference vlaue is {difference} floor value {floor_value} and remainder value {remainder}");
    
    // Booleans are one byte of size 
    
    let t = true;
    // type annotation 
    let f: bool = false;
    // char literal with single quotes
    let character:char = 'Z';
    
    println!("t {t} f {f} and character is {character}");
    
    // Compound types group multiiiiiple values in one type [tupel and arrays]
    
    let tup: (i32, f64, u8) = (500, 5.5, 90);
    // destucturing
    
    let (_x, _y, _z) = tup;
    println!("the value of y is {}", _y);
    
    // . notation access with index
    
    let x:(i32, f64, u8) = (-232434, 4.33, 33);
    
    let signed_number = x.0;
    let float_number = x.1;
    let number = x.2;
    
    println!("signed number : {signed_number} floating point number: {float_number} number only: {number}");
    
    // array must have same type
    
    let a:[i32;4] = [1,2,3,4];
    let first_value = a[0];
    println!("arry a {first_value}");
    // will generate an error
    // let b = [2,2.3];
    // println!("arry b {b}");
    let months=["januay", "february"];
    
    
    let first_month = months[0];
    println!("Print month array {first_month}");
    
    let i = ([1;2], [3;4]);
    let (ai, _) = i;
    
    println!("sumation of array item is {}", ai[0] +i.1[0])
}