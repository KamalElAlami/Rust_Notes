fn main() {
    // variables here by default is constant 
    let age = 16;
    // if you need changeable var you have to add mutable keyword
    let mut agee = 0;
    agee = 16;
    // primary datatypes : we have float, boolean, char, int
    // Integers ⚡
        let i: i8;    // -128 to 127
        let i: i16;   // -32,768 to 32,767
        let i: i32;   // -2³¹ to 2³¹-1 (default)
        let i: i64;   // -2⁶³ to 2⁶³-1
        let i: i128;  // -2¹²⁷ to 2¹²⁷-1
        let i: isize; // Depends on architecture (64 or 32 bits)

    // Unsigned Integers 🔢
        let u: u8;    // 0 to 255
        let u: u16;   // 0 to 65,535
        let u: u32;   // 0 to 4,294,967,295
        let u: u64;   // 0 to 18,446,744,073,709,551,615
        let u: u128;  // 0 to 2¹²⁸-1
        let u: usize; // Depends on architecture
    // Floating Point 💫
        let f: f32;   // Single precision
        let f: f64;   // Double precision (default)

    // Boolean 🎯
        let b: bool;  // true or false

    // Character 📝
        let c: char;  // Unicode character (4 bytes)
    // strings kay two types 
        // string
        let str = "Salam".to_string();
        let str: String = String::from("Kamal"); // String normalment hadi kadir wa7eed dynamic allocation fl heap
        let mut s2 = String::from("Hello");
        // slice
        let str = "Kamal"; // slice katkun endha allocation static fl stack

        ///////////////// Compound Data Types //////////////////////
        /// we have arrays 
        let my_arr = [1, 2, 3, 4, 5, 6];
        let my_ar: [i32] = [1, 2, 3, 4, 5, 6];
        /// we also have tuples
        let my_tuple = ("kamal", 'c', 25);
        let my_tuple1: (String, char, i32) = ("kamal", 'c', 25);
}
