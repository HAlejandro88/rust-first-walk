fn main() {
    call_me();
    another_function(8);
    print_labeled_measurement(5,'h');
    inside_variable_into_another_side();
}

fn call_me() {
    println!("creado desde otra funcion")
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}


fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn inside_variable_into_another_side() {
    let y = { // si la expresion termina en ";"
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}