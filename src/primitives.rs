use std::fmt;
// Scalar types
// Signed integers : i8, i16, i32, i64, i128 and isize (pointer size)
// Unsigned intergers : u8, u16, u32, u64, u128 and usize (pointer size)
// Floating point : f32, f64
// Unicode scalar value : char (4 bytes)
// Boolean : bool

// Compound types
// Arrays like [1, 2, 3]
// Tuples like (1, true)

// Tuples can be used as function arguments and as return values.
fn reverse(pair: (i32, bool)) -> (bool,i32) {
    let (int_parem, bool_param) = pair;
    (bool_param, int_parem)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
    }
}

fn transpose(matrix: Matrix) -> Matrix {
    let copy = Matrix(matrix.0, matrix.2, matrix.1, matrix.3);
    copy
}
 
fn main() {
    let logical: bool = true; // Assign type

    let a_float: f64 = 1.3; // Regular annotation
    let an_integer = 5i32; // Suffix annotation

    let default_float = 4.3; // f64
    let default_integer = 4; // i32

    let mut mutable = 12; // Mutable i32
    mutable = 345;

    //mutable = true; // Error! Type cannot be changed

    let mutable = true; // Overwritten with shadowing

    // Literals and operators
    // Integers can be expressed using : 0x (hexa), 0o (octal) or 0b (binary)

    println!("1 + 2 = {}", 1u32 + 2);
    println!("1 - 2 = {}", 1i32 - 2); // Type is important! 

    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3); // Scientific notation

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is 0b{:b}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);

    // Tuples
    // A tuple is a collection of values of different types.
    // A tuple with a bunch of different types.
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
        -1i8, -2i16, -3i32, -4i64,
        0.1f32, 0.2f64,
        'a', true);

    // Values can be extracted from the tuple using tuple indexing.
    println!("Long tuple first value: {}", long_tuple.0);
    println!("Long tuple second value: {}", long_tuple.1);

    // Tuples can be tuple members.
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // Tuples are printable.
    println!("tuple of tuples: {:?}", tuple_of_tuples);
    // But long Tuples (more than 12 elements) cannot be printed.

    let pair = (1, true);
    println!("Pair is {:?}", pair);
    println!("The reversed pair is {:?}", reverse(pair));

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}",transpose(matrix));   

    // Arrays and Slices

    // Fixed-size array (type signature is superfluous).
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized to the same value.
    let ys: [i32; 500] = [0; 500];

    // `len` returns the count of elements in the array.
    println!("Number of elements in array: {}", xs.len());

    fn analyze_slice(slice: &[i32]) {
        println!("First element of the slice: {}", slice[0]);
        println!("The slice has {} elements", slice.len());
    }

    // Arrays can be automatically borrowed as slices.
    println!("Borrow the whole array as a slice.");
    analyze_slice(&xs);

    // Slices can point to a section of an array.
    // They are of the form [starting_index..ending_index].
    // `starting_index` is the first position in the slice.
    // `ending_index` is one more than the last position in the slice.
    println!("Borrow a section of the array as a slice.");
    analyze_slice(&ys[1 .. 4]);
}
