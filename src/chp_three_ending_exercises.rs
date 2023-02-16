pub fn convert_f_to_c(far : f64) -> f64 {
    //(far − 32) × 5/9 
    let cel: f64 = (far - 32_f64) as f64 * (5.0_f64 / 9.0_f64);
    return cel;
}

pub fn convert_c_to_f(cel : f64) -> f64 {
    //(cel × 9/5) + 32 = 32°F
    let far: f64 =  (cel * 9.0_f64/5.0_f64) + 32_f64;
    return far;
}

pub fn fibonacci_series_while_loop(n: u64) -> u64 {
    let mut first_num = 0;
    let mut second_num = 1;
    
    if(n < 3 && n != 0) {
        return n - 1;
    } else if (n == 0){
        println!("YOU HAVE ENTERED AN INVALID NUMBER!");
        return 0;
    }

    let mut counter: u64 = 3;
    let mut currentNum: u64 = 0;
    while(counter <= n){
        currentNum = first_num + second_num;
        first_num = second_num;
        second_num = currentNum;

        counter += 1;
    }

    return currentNum;
}

pub fn fibonacci_series_for_loop(n: u64) -> u64 {
    let mut first_num = 0;
    let mut second_num = 1;
    
    if(n < 3 && n != 0) {
        return n - 1;
    } else if (n == 0){
        println!("YOU HAVE ENTERED AN INVALID NUMBER!");
        return 0;
    }

    
    let mut currentNum: u64 = 0;
    for counter in (3..n + 1){
        currentNum = first_num + second_num;
        first_num = second_num;
        second_num = currentNum;
    }

    return currentNum;
}

