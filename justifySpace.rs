fn main() {
    // You can right-justify text with a specified width. This will
     // output "    1". (Four white spaces and a "1", for a total width of 5.)
    println!("{number}", number=4);
    println!("{number:>2}", number=4);
    println!("{number:>3}", number=4);
    println!("{number:>4}", number=4);
    println!("{number:>5}", number=4);
    println!("{number:>8}", number=4);
 }