use std::fmt;

// This is the main function.
fn main() {
    // Statements here are executed when the compiled binary is called.

    // Print text to the console.
    println!("Hello World!");

    // Printing is handled by a series of macros defined in std::fmt

    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31);

    // Positional arguments can be used. Specifying an integer inside `{}`
    // determines which additional argument will be replaced. Arguments start
    // at 0 immediately after the format string.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // Different formatting can be invoked by specifying the format character
    // after a `:`.
    println!("Base 10:               {}",   69420); // 69420
    println!("Base 2 (binary):       0b{:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        0o{:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): 0x{:x}", 69420); // 10f2c

    // You can right-justify text with a specified width. This will
    // output "    1". (Four white spaces and a "1", for a total width of 5.)
    println!("{number:>5}", number=1);

    // You can pad numbers with extra zeroes,
    println!("{number:0>5}", number=1); // 00001
    // and left-adjust by flipping the sign. This will output "10000".
    println!("{number:0<5}", number=1); // 10000

    // You can use named arguments in the format specifier by appending a `$`.
    println!("{number:0>width$}", number=1, width=5);

    #[derive(Debug)]
    struct Structure(i32);

    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");

    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3));

    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?
    println!("Now {:?} will print!", Structure(7));

    // Pretty print
    println!("{:#?}", Structure(9));

    impl fmt::Display for Structure {
        // This trait requires `fmt` with this exact signature.
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Write strictly the first element into the supplied output
            // stream: `f`. Returns `fmt::Result` which indicates whether the
            // operation succeeded or failed. Note that `write!` uses syntax which
            // is very similar to `println!`.
            write!(f, "My Strucure is {}", self.0)
        }
    }

    println!("{}", Structure(8));

    // Define a structure where the fields are nameable for comparison.
    #[derive(Debug)]
    struct Imaginary {
        re: f64,
        im: f64,
    }

    // Similarly, implement `Display` for `Point2D`.
    impl fmt::Display for Imaginary {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Customize so only `x` and `y` are denoted.
            write!(f, "{} + {}i", self.re, self.im)
        }
    }

    let point = Imaginary{ re:2.4 , im:9.4 };
    println!("Compare points:");  
    println!("Display: {}", point);
    println!("Debug: {:?}", point);
    
}