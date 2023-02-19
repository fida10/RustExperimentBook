pub fn why_use_references(){
    // let x = String::from("Hello there");
    // ownership_function(x);
    // println!("{}", x);

    // these three lines of code will not work, due to ownership in rust.
    // when x is passed into ownership_function, that function takes possesion of x
    // so we cannot use x anymore anywhere outside of the ownership_function
    // unless we return it
    // this only works with compound types like strings and boxes (see below for examples of those)
    // does not work for primitive types like integers
}

pub fn free_function(){
    // let b = Box::new("Hi");
    // let a = b;

    // free(b); //free function does not exist in rust
    //memory cannot be freed manually
    //must be done by reallocating the heap memory (value of "Hi") to another variable
    // println!("{}", b);
}

pub fn ownership_function(x: String){
    println!("{}", x);
}

pub fn reference_fun(){
    let mut x: Box<i32> = Box::new(1); //this creates a new value within memory heap itself
    let a: i32 = *x; //this de-references, meaning it assigns the heap value of x directly to a
    *x += 1; //this adds 1 to the value of x in memory. The asterisk is there because this is a dereference. 
    //if it were a reference, we could not add to it (i.e. if it were 'x' directly, reason being is that x directly is a "Box" not an int)
    // '*' allows us to access the value within heap directly and get around the box

    let r1: &Box<i32> = &x; //r1 points to the reference of x (hence the '&'), which in turn points to the value of 'x' (2)
    //r1 points to the stack ('x'), not a value within the heap
    let b: i32 = **r1; //r1 has 2 stars in front of it 
    //reason being is that r1 is itself a reference to &x, which in turn is a reference to the value of x
    //so we have 2 orders of references, and thus we need two stars

    let r2: &i32 = &*x; //r2 has &* in front of it, and the type is &i32
    //reason being is that r2 is a reference to the value of x (which is why we put *x)
    //since this is a reference, we must put &i32 as the type

    let c: i32 = *r2;
    //we know that * refers to the value of whatever variable comes after it
    //so basically this will mean the value of r2 gets stored inside c (which is 2)

    println!("a is {} x is {} *x is {} r1 is {} b is {} r2 is {} c is {}", a, x, *x, r1, b, r2, c);
    println!("a is {}", a); //a will still be 1. in the above line, we assigned 'a' to the value of x (*x)
    //so now this value is assigned directly to a, and only a, in the heap (1 in the heap, a in the reference)
    //thus, even if we change the value of x, a remains unchanged because a has an independant heap value to x
}

pub fn more_reference_fun(){
    let x: Box<i32> = Box::new(-1);
    let x_abs1 = i32::abs(*x);
    let x_abs2 = x.abs();
    assert_eq!(x_abs1, x_abs2);

    let r: &Box<i32> = &x;
    let r_abs1 = i32::abs(**r);
    let r_abs2 = r.abs();
    assert_eq!(r_abs1, r_abs2);

    let s = String::from("Hello");
    let s_len1 = str::len(&s); //str::len() method requires a reference
    //reason being is for ownership in rust
    //if it took the string directly in the arguments, we could not use the 's' variable outside of the len method
    //which would be inconvenient

    let s_len2 = s.len();
    assert_eq!(s_len1, s_len2);
}

pub fn mutability_and_aliasing(){
    let mut x = String::from("Hello"); //creates x and assigns value as "Hello". 
    //x is mutable at this point

    let y: &String = &x; //y references the value of x.
    //Because 'y' is not mutable, we cannot change the value of y
    //however, the value of 'x' can still be changed

    println!("{} and {}", x, y); //will print "hello" and "hello". This also limits the scope of 'y' to this println
    //to extend its scope, we must reference it again

    x.push_str(" world"); //changes the value of "x" to "hello world"
    //println!("{}", y); //this and any reference to y will cause an issue
    //because y is immutable and the value of y is a reference to x, y can now never be called
    //reason being is that 'x' has changed BUT 'y' does not allow a change, 
    //so to reference y would take away 'x's write privelages and cause an error on the push_str line
    //so it cannot be called
}

pub fn making_references_mutable(){
    //very similar to above function
    let mut x = String::from("Hello");
    let y: &mut String = &mut x; //here we initialize y as a mutable string reference
    //& -> reference, mut -> changeable (writeable)
    
    y.push_str(" world"); //therefore, y has write privelages
    println!("{}", x);

    //y.push_str(" universe"); //This will cause an issue, due to the golden rule
    //should not be aliasing and mutating at the same time
    //this is dangerous since now you can change the value in the heap from two separate references
    //breaks rust golden rule number
    //also, 'y's scope ends as it is overwritten by x reference on println! line;

    //now if I want the value of 'x', I MUST reference it from 'x' because of the golden rule
    //referencing 'y' here will cause errors 
}

pub fn making_mutable_references_unmutable(){
    let mut x = String::from("Hello");
    let y = &mut x;
    let z = &*y;
    println!("{} {}", y, z);
        
    //x.push_str(" and greetings!");
    y.push_str(" but not happy greetings!");
    println!("{}", y);
    println!("{}", x);

    //why do this? We could return 'z', which is readable only, while 'y' is only within the scope of this function
    //therefore, 'y' can only be changed in the scope of this function
    //wheras z, what is returned to other functions, is read only
    //example: bank account
}

fn add_ref(v: &mut Vec<&i32>, n: i32) {
    //let r = &n; //this line will cause an error
    //the reason being is that &n is a reference within a function

    //v.push(r);
    //this line pushes 'r', which is &n
    //however, &n and by extension 'r' will die at the end of this function
    //'v', the mutable vector on the other hand, will continue to exist outside of this function
    //so an element within 'v' will no longer exist at the end of the function
    //so we will get a "does not live long enough error" on the "let r = &n" line
}

pub fn plus_one(x: i32) -> i32 {
    x + 1
}