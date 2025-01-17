/* --- BEGIN Variable Definitions ---
Struct f{x,y};
Owner _y;
Function String::from();
Function println!();
--- END Variable Definitions --- */
struct Foo {
    x: i32,
    y: String,
}

fn main() {
    let _y = String :: from("bar"); // !{ Move(String::from()->_y) }
    let f = Foo { x: 5, y: _y }; // !{ Bind(None->f), Bind(None->f.x), Bind(_y->f.y) }
    println!("{}", f.x); // !{ PassByStaticReference(f.x->println!())  }
    println!("{}", f.y); // !{ PassByStaticReference(f.y->println!())  }
} // !{ StructBox(f->f.y), GoOutOfScope(f), GoOutOfScope(f.x), GoOutOfScope(f.y) }