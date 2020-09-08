extern crate mylib;

fn main() {
    let mut a = mylib::Vahho { data: 200 };
    println!("{}", a.get_size());
    a.add_value(100);
    println!("{}", a.get_size());
}
