use euler::get_factor_count_i32;

fn main() {
    let test_size = false;
    let mut counter = 1i32;
    let mut triangle = 0i32;
    let mut tri_max = 0i32;
    while test_size == false {
        triangle = triangle+counter;
        let tri_val = get_factor_count_i32(triangle);
        if tri_val > tri_max {
            println!("Max: {} - {}", tri_val, triangle);
            tri_max = tri_val;
        }
        if tri_val > 500 {
            println!("Triangle: {}", triangle);
            break;
        }
        counter+=1;
    }
}

