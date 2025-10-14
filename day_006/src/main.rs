
struct SafeNumber {
    ptr : *mut i32 // Raw pointer to i32
}

impl  SafeNumber {
    fn   new(value:i32)->Self{
        let boxed = Box::new(value); // Safe allocation
        let ptr = Box::into_raw(boxed);  //Convert to  raw pointer
        SafeNumber { ptr }
    }

    fn get(&self)->i32{
        unsafe {*self.ptr}
    }
     fn set(&mut self, value: i32) {
        unsafe { *self.ptr = value; } // Safe update
    }
}

impl Drop for SafeNumber {
    fn drop(&mut self) {
        unsafe { Box::from_raw(self.ptr); } // Reclaim memory
    }
}



fn main() {

    let  mut num = SafeNumber::new(42);
     println!("Value: {}", num.get()); // Prints 42
     num.set(100);
     println!("Updated: {}", num.get()); // Prints 100
    let mut data = 42;
    let ptr: *mut i32 = &mut data; // Raw mutable pointer
    let ptr2: *const i32 = &mut data;
     println!("Updated: {}", num.get()); // Prints 100

    unsafe {
        *ptr = 3; // Dereference to update
        println!("Read via ptr2 :{}",*ptr2);
    }
    println!("Data: {}", data); // Prints 100
}