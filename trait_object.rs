pub struct TraitObject {
    pub data: *mut (), // point to the concrete type data
    pub vtable: *mut (); // list of function pointers
}

// https//doc.rust-lang.org/std/raw/struct.TraitObject.html
// https://geo-ant.github.io/blog/2023/rust-dyn-trait-objects-fat-pointers/
// https://rust-book.junmajinlong.com/ch11/04_trait_object.html
