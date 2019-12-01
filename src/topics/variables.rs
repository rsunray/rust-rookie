pub fn demo_variables() {
    let immutable_var: i32 = 5;
    println!("initial value of immutable_var : {}", immutable_var);

    let mut mutable_var: i32 = 5;
    println!("mutable_var : {}", mutable_var);
    mutable_var = 6;
    println!("mutated mutable_var : {}", mutable_var);

    let mut immutable_var: i32 = 7;
    println!("before mutating: {}", immutable_var);
    immutable_var = 3;
    println!("making an immutable var to mutable: {}", immutable_var);

    let _shadow_var = 2;
    let _shadow_var = "3";
    println!("after shadowing shadow_var : {}", _shadow_var);
}
