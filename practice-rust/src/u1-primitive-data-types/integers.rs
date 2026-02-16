// Rust has signed and unsigned integer data types.
// Signed Types: i8, i16, i32, i64, i128
// Unsigned Types: u8, u16, u32, u64, u128

// 8, 16, 32, 64, 128 refer to bit lengths
// the larger the number of bits the larger the max and min (if signed) value it can store.

// Float types: f32, f64
// bool type
// char type

fn main()
    {
        // let declares a variable (default immutable (cannot be changed) after initialization)
        // mut turns it into a mutable variable

        let mut a: u32; //declaration without initialization
        let mut b: i32 = 100; //declaration with initialization

        b = 6; // valid
        b = -7; // also valid, negative value with signed integer

        a = 6; // valid
        // a = -7; // invalid! negative value used with unsigned integer type.

            /*

                error[E0600]: cannot apply unary operator `-` to type `u32`
                --> integers.rs:22:13
                |
                22 |         a = -7; // invalid! negative value used with unsigned integer type.
                |             ^^ cannot apply unary operator `-`
                |
                = note: unsigned values cannot be negated

                error: aborting due to 1 previous error

            */

        a = 7; // valid, positive value with unsigned integer

        println!("Value of a: {}", a);
        println!("Value of b: {}", b);

        let c: bool = a == 7;
        println!("Is a equal to 7?: {}", c);

        let character: char = 'c';
        println!("Character: {}", character);

        /*

            Expected Final Output:

            Value of a: 7
            Value of b: -7
            Is a equal to 7?: true
            Character: c

        */
    }

