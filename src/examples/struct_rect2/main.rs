/* --- BEGIN Variable Definitions ---
Struct r{width, height};
StaticRef rect;
StaticRef self;
Function area();
Function print_area();
--- END Variable Definitions --- */
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 { // !{ InitializeParam(self) }
        self.width * self.height
    } // !{ GoOutOfScope(self) }
}

fn print_area(rect: &Rectangle) { // !{ InitializeParam(rect) }
    println!(
        "The area of the rectangle is {} square pixels.",
       	rect.area() // dot even though it's actually a reference !{ PassByStaticReference(rect->area()) }
    );
} // !{ GoOutOfScope(rect) }

fn main() {
    let r = Rectangle { // !{ Bind(None->r) }
        width: 30, // !{ Bind(None->r.width) }
        height: 50, // !{ Bind(None->r.height) }
    };
    
   	print_area(&r); // !{ PassByStaticReference(r->print_area()) }
} // !{ StructBox(r->r.height), GoOutOfScope(r), GoOutOfScope(r.width), GoOutOfScope(r.height) }