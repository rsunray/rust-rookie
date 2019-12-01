pub fn demo_control_flow() {
    // 'if' and 'else' are expressions not statements
    exp_if();

    exp_if_assingment();

    exp_if_else();

    exp_if_else_if();

    repetition_loop();

    repetition_while();

    repetition_for_in();
}

fn exp_if() {
    let x: bool = true;

    if x {
        println!("this is an if block")
    }
}

fn exp_if_else() {
    let y: bool = false;

    if y {
        println!("this is an if block");
    } else {
        println!("this is an else block");
    }
}

fn exp_if_else_if() {
    let x: bool = false;
    let y: bool = true;

    if x {
        println!("this is an if block");
    } else if y {
        println!("this is an  else if block");
    } else {
        println!("this is an else block");
    }
}

fn exp_if_assingment() {
    let x: bool = true;

    let y: u32 = if x {
        2
    } else {
        1
    };

    println!("conditional assignment result : {}", y);
}

fn repetition_loop() {
    // TODO :- will add the demo once Operation topic is complete
}

fn repetition_while() {
    // TODO :- will add the demo once Operation topic is complete
}

fn repetition_for_in() {
    // TODO :- will add the demo once Operation topic is complete
}

