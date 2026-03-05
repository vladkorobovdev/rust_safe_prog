fn main() {
    let mut ascii_number: u8 = 33;
    while ascii_number < 127 {
        println!("{:?} : {ascii_number}", ascii_number as char);
        ascii_number += 1;
    }
}