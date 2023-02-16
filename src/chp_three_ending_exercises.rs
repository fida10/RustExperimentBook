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

