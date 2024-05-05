#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None; 
    }
    if num == 1 {
        return Some(Classification::Deficient);
    }

    let mut div_sum = 1;
    for i in 2..(num){
        if num%i==0{
            div_sum += num/i;
        }
    }
    println!("{}", div_sum);
    match div_sum {
        div if div == num => Some(Classification::Perfect),
        div if div > num => Some(Classification::Abundant),
        _ => Some(Classification::Deficient),
    }
    
    
    //unimplemented!("classify {num}");
}