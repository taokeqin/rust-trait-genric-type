pub fn sum_int_array(input_array: &[u32]) -> Option<u32>{
    let mut sum: u32 = 0;
    for i in input_array {
        let new_sum = sum.checked_add(*i);
        if new_sum.is_none(){
            return None
        }
        sum = new_sum.unwrap()
    }
    Some(sum)
}