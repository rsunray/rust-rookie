pub fn demo_functions() {
    let _no_return_empty_tuple = simple_fn();

    let _no_return_empty_tuple = parameterized_fn(2048);

    let two_kilo_bytes: u64 = producer_fn();

    let (two, one_kilo_byte) = transformer_fn(2,1024);

    let no_of_words = {
        512    // expression
    };

    println!("All returned values : {} {} {} {} {}", two_kilo_bytes, two, one_kilo_byte,two_kilo_bytes, no_of_words);
}

fn simple_fn() {
    println!("this is the simplest function returning nothing");
}

fn parameterized_fn(param1: u64) {
    println!("this is a parameterized function with : {}", param1);
}

fn producer_fn() -> u64 {
    2048
}

fn transformer_fn(param1: u64, param2: u64) -> (u64 , u64){
    (param1, param2)
}