pub fn capacitancia(cap: vec!<u64>) -> f64 {
    let result: f64  = 0.0;
    for(x = 0; x < cap.length; x++){
        result += cap[x];
    }
    result
}