use std::mem::drop;

struct CustomSmartPointer {
    data: String
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}


fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff")
    };

    // error[E0040]: explicit use of destructor method
    // 不允许显式调用 `drop`. 因为仍会在结尾自动调用 drop
    // c.drop();
    
    let d = CustomSmartPointer {
        data: String::from("other stuff")
    };

    println!("CustomSmartPointers created.");
    // 使用 std::mem::drop
    drop(c);
    println!("CustomSmartPointers dropped before the end of main.");
}
