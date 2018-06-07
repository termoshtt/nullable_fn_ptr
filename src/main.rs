extern "C" fn callback(a: i32) {
    println!("I'm called from C with value {0}", a);
}

#[link(name="ext")]
extern "C" {
    fn register_callback(cb: Option<extern "C" fn(i32)>) -> i32;
    fn trigger_callback();
}

fn main() {
    unsafe {
        register_callback(Some(callback));
        trigger_callback();
    }
    unsafe {
        register_callback(None);
        trigger_callback();
    }
}
