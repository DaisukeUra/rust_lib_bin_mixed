#[no_mangle]

pub struct Vahho {
    pub data: i32,
}

impl Vahho {
    pub fn get_size(&self) -> i32 {
        self.data
    }

    pub fn add_value(&mut self, value: i32) {
        self.data += value;
    }
}

pub extern "C" fn Registration() {
    let mut a = Vahho { data: 200 };
    println!("{}", a.get_size());
    a.add_value(10);
    println!("{}", a.get_size());
    a.add_value(300);
    println!("{}", a.get_size());
}
