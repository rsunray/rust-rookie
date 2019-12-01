pub fn demo_data_types() {
    scalar();

    compound();
}

fn scalar() {
    let uint: u8 = 255; // panics with overflowing error for 256
    println!("unsigned integer : {}", uint);

    let sfloat: f32 = -1.628;
    println!("signed 64 bit float {}", sfloat);

    let fbool: bool = false;
    println!("false boolean {}", fbool);

    let smile_emoji_char: char = '\u{1F60A}';
    println!("smiley {}", smile_emoji_char);
}

fn compound() {
    let tup: (u8, f32, bool, char) = (255, -1.68, false, '\u{1F60A}');
    println!("tuple element access {}", tup.0);

    let (uint, sfloat, fbool, smile_emoji_char) = tup;
    println!("tuple destructuring {} {} {} {}", uint, sfloat, fbool, smile_emoji_char );

    let arr: [u32; 5] = [32,33,34,35,36];
    println!("array access as usual {}", arr[0]);
}