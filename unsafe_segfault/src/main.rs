use std::marker::PhantomPinned;

struct Container {
    data :String,
    ptr  :*mut String,
    _pin :PhantomPinned /* Ensures !Unpin (stop the compiler from automatically applying Unpin) */
}

impl Container {
    fn new() -> Self {
        let mut container = Container { data: "".to_string(), ptr: std::ptr::null_mut(), _pin: PhantomPinned };
        container.ptr = &mut container.data; /* container is on the stack, hence ptr points to a stack value */

        /* container is moved to a different stack (return value), ptr is not pinned! */
        container
    }

    /* get() and set() are undefined behaviour! */
    fn get(&self) -> &str {
        let ret :&str;
        unsafe {
            ret = (*self.ptr).as_str();
        }

        ret
    }

    fn set(&self, val :String) {
        unsafe {
            *self.ptr = val;
        }
    }
}


fn main() {
    let container = Container::new();
    println!("data: {}", container.get());

    container.set("hello world".to_string());
    println!("data: {}", container.get());
}
