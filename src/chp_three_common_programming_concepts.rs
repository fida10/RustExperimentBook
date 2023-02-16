pub fn another_function(x: i32){
    println!("This is another function, it will print {}", x);
}

pub fn printTwoParam(varOne: i32, varTwo: f64){
    println!("{} {}", varOne, varTwo);
}

pub fn returnSomething() -> bool {
    true
}

pub fn returnNumber() -> i32 {
    let x = 45;
    x + 43 - 4
}

pub fn returnNumberWithParameter(varOne: i32) -> i32 {
    varOne + 5 + 6
}

pub fn if_statement(x: i32){
    if x > 20 {
        println!("X is larger than 20");
    } else if x == 20 {
        println!("X is equal to 20");
    } else {
        println!("X is less than 20");
    }
}

pub fn if_statement_with_let() -> i32{
    let varOne = true;
    let varTwo = if varOne {5} else {6};
    varTwo
}



pub fn loop_example(mut varOne: i32){

    loop{
        let x = varOne <= 0;
        if(x){
            break;
        }
        println!("Hello!");
        varOne -= 1; 
    }

    while varOne != 0 {
        println!("Hello!");
        varOne -= 1;
    }
}

pub fn loop_example_two(){
    let a = [55, 63, 21, 33, 66, 10];

    for indivElem in a {
        println!("{}", indivElem);
    }

    for indivElement in (1..25).rev(){
        println!("{}", indivElement);
    }
}


pub fn question_two() {
    let a = [5; 10]; //[5, 5, 5, 5, 5, 5, 5, 5, 5, 5]
    let mut sum = 0;
    for x in a {
        sum += x;
    }
    println!("{sum}");
}