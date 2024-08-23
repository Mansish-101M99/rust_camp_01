use std::io;

fn main() {
    // println!("Hello, world!");

    // Rust is a statically-typed language.

    /*
    Prelude :- List of things that Rust automatically imports (no manual imports) into every Rust program. It's kept as small as possible and is focused on things, particularly traits, which are usedd in almost every Rust program.

    Crate :- A package of code or collection of code (packages or libraries)
    Module :- A specific piece of functionality (A function within a code/file)

    e.g.> use std::io; >--> where 'std' is Standard package/library and 'io' is input/output.
          '::' is a path specifier and acts as a dot operator to access it.
    */


    /* We can only do Arithmetic operations only on variables and values of same datatype. Keep the MAX condition in mind also. */


    // Implicit variable - We can't change the type of this variable throughout the program. Here we don't specify the type of variable, but during compile time the compiler assigns the datatype to the variable as per the data is stores.
    let x = 5;

    // Explicit variable - We assign the datatype by ourselves, e.g., let <variable>: u32 = <value>;
    let y: u32 = 10;

    /*  Rust variables are immutable; we cannot change the value within it as in the given example-
    let o = 4;
    o = 3;
     To iterate/change it we use 'mut' keyword. */
    // let mut z = 2;
    let z = 2;

    // variables / functions within a set local scope as shown here can have any value irrepective of the items in global. We can access global scope items within this local scope but the local items can't be accessed via global.
    {
        let x = 6;
        let y = y - 4;
        println!(
            "X and Y within the local variable of curly braces : {} & {}",
            x, y
        );
    }

    println!(
        " (x):{} + (y):{} * (z):{} = (x+y*z):{}",
        x,
        y,
        z,
        (x + y * z)
    );

    let x = x + 2;
    println!("x is Thala {}", x);

    // Unlike mutability with 'mut' keyword, we are redefining the variable 'z' here. So we can use the'let' keyword and redefine the variable 'z' with the type '&str'.
    let z = "Now a str";
    println!("Now z is str : {}", z);

    // Constant varaible or variables with a 'const' keyword have a fixed datatype and value; their datatype and values are not changable. We can initiate const variables by all capital alphabets with underscores to form snake-case text.
    const ONLYCAPS: u32 = 91;
    const ONLYCAPS_TWO: u32 = 28;
    println!(
        "Two cons variables values : {} & {}",
        ONLYCAPS, ONLYCAPS_TWO
    );

    let x: u8 = 5;
    let y = x;
    println!("x: {} & y: {}", x, y);
    // Now y will also become of type 'u8' and changing it will cause error.

    /*
    Scaler Data-type : Something with finite set of possible values (following some scale). Each value can be compared to other values as either equal, greater or lesser.  [Integer (int, uint), Floating-point (float), Boolean (bool), character (char)]..

      Integers ::--  All types of integers with no decimal points. i32 and u32 are by default in Rust.
                   i -> (Assigned integers)         &         u -> (Unassigned integers)
                    i8, i16, i32, i64, i128         ||         u8, u16, u32, u64, u128
            For example if we take i8 and u8 as reference ...
            Range for i8: (-128 to 127) -> ((-2^7 - 1) to (2^7 - 1))   |||   u8: (0 to 255) -> (0 to (2^8 - 1))

      Floating_point ::-- f64 is the default float identifier for unassigned type.  [f32 (single-precission), f64 (double-precission)]

      Boolean ::--  true <=> 1  &  false <=> 0        let <var>: bool = <value>;

      Character ::--  let <var>: char = '<single_character_on_the_board>';

    ----------------------------------------------------------------------------------------------------------------------------------------

    Compound Data-type : Data-types that can be constructed using the scaler/primitive datatypes. [tuples, arrays]

      Tuples ::-- These are fixed length sequence of elements which are immutable. We can use 'mut' to make elements mutable. It is stored
      as fixed-size contiguous block memory on the stack. Collection of different datatypes in a single compound value.

      Arrays ::-- These are fixed length sequence of elements which are immutable consisting elements of similar datatype and stored as contiguous block in stack memory as its size is known during compile time. We can use 'mut' to make elements mutable.
    */



    // We can access tuple elements within the print statements via indexing of the tuple variable.
    // println!(" {} {} .... {} ", tuple.0, tuple.1, .... , tuple.n);  <<<\ where {0, 1, ..., n} are index numbers as per variable.

    let mut tuple_var: (i32, f64, bool, char) = (6, 53.89, true, 'h');
    let tuple_var_2: (i8, f32, bool, char) = (6, 53.89, true, 'h'); // The numeric values of both variable ahve different datatypes.

    println!(
        "A tuple_var -> ( {}, {}, {}, {} )",
        tuple_var.0, tuple_var.1, tuple_var.2, tuple_var.3
    );
    println!(
        "A tuple_var_2 -> ( {}, {}, {}, {} )",
        tuple_var_2.0, tuple_var_2.1, tuple_var_2.2, tuple_var_2.3
    );

    tuple_var.1 = 48.534;
    println!(" New float value for index 1 : {}", tuple_var.1);

    tuple_var = (41, 27.012, false, 'y');
    println!(
        " New values for tuple_var : ( {}, {}, {}, {} )",
        tuple_var.0, tuple_var.1, tuple_var.2, tuple_var.3
    );

    // Examples >> 1 
    // Tuples can be tuple's members
    let _t1: (u8, (i16, u32)) = (0, (-1, 1));
    let _t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));

    // 2 
    /* If there are more than 12 elements in a tuple, it can't be printable. It can have those elements but can't print at once. */
    let _too_long_tuple_p1 = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    println!("too long tuple: {:?}", too_long_tuple);

    // 3 
    let (x, y, z);   
    (y, z, x) = (1, 2, 3);      // Destructuring assignments
    
    assert_eq!(x, 3);
    assert_eq!(y, 1);
    assert_eq!(z, 2);

    // 4 
    let (x, y) = sum_multiply((2, 3));
    assert_eq!(x, 5);
    assert_eq!(y, 6);
    println!("Sum = {}, Multiply = {}", x, y);



    // let array: [<datatype>; <number_of_elements>] = [<all_elements_with_given_datatype>];  
    // The above indicates that the length is fixed during compile time. They don't have changable size and must retain their size. 
    let mut arr1 = [1, 2, 3, 4, 5, 6, 7];

    // println!(" {} {} .... {} ", array[0], array[1], .... , array[n]);  <<<\ where {0, 1, ..., n} are index numbers as per variable.
    println!("arr1 random index : {}", arr1[3]);

    arr1[5] = 10;
    println!("arr1 new element at index 5 : {}", arr1[5]);

    // Examples >> 1 
    /* We can ignore parts of the array type or even the whole type, let the compiler infer it for us */
    let arr0 = [1, 2, 3];
    let arr: [char; 4] = ['a', 'b', 'c', 'd'];
    for c in arr.iter() {
        print!("{}, ", c);
    }
    println!("\n[ {}, {}, {} ]", arr0[2], arr0[1], arr0[0]);
    
    /* Arrays are stack allocated, `std::mem::size_of_val` returns the bytes which an array occupies */
    // A char takes 4 bytes in Rust: Unicode char
    assert!(std::mem::size_of_val(&arr) == 16);
    println!("");

    // 2
    /* All elements in an array can be initialized to the same value at once */
    let list: [i32; 100] = [1; 100];    // Same as [1,1,..1 > 100 times]

    assert!(list[0] == 1);
    assert!(list[65] == 1);
    assert!(list.len() == 100);

    // 3  
    let names: [String; 2] = [String::from("Sunfei"), "Sunface".to_string()];
    
    /* `Get` returns an Option<T>, it's safe to use */
    let name0 = names.get(0).unwrap();
    println!("Using get() method , {}", name0);

    // Indexing is unsafe compared to get()
    let name1 = &names[1];
    println!("Using indexing <arr>[<index>] method , {}", name1);

    

    /*
    String -|- &str
    > String -- it is a heap-allocated string type that owns its contents and is mutable. If you need to own the data and mutate it, use String. This type is defined in std and stored as a vector of bytes (Vec), but guaranteed to always be a valid UTF-8 sequence. String is heap allocated, growable and not null terminated.

    > &str -- It is an immutable sequence of UTF-8 Bytes in memory, it doesnot own the underlying data and is immutable. This can be seen/viewed as a sequence of characters (stored in UTF-8 Bytes) in memory. &str is more light-weight and efficient than String. 

    String Literal -- It's a sequence of characters enclosedd within "<content>". It is an other form of &str datatype.
    -> It has fixed size with compile-time known sequence of UTF-8 Bytes. 
    -> This type is `&'static str`, which indicates that this data is stored in static storage, meaning it is valid throughout the entire lifetime of program. 
    -> The data is hardcoded into executable and stored in read-only memory, meaning they are immutable.
    */

    // Examples >> 1
    let s1: Box<str> = "Hello World Again..!!!".into();    // .into() -> converts the type into the type in variable, which we are annotating
    // Box<str> ; we are annotating the data within a heap-allocated variable.
    greet_re(&s1);
    println!("{}", s1);

    // 2
    let mut s2 = String::from("hello");
    s2.push(',');                  // here, push() will push a single character to the String s2
    s2.push_str(" world");     // here, push_str() will push a string literal to the String s2
    s2 += "!";                        // += can also be used to annotate a String
    println!("{}", s2);

    // 3
    let s3 = String::from("I like dogs");
    println!("{}", s3);
    // Allocate new memory and store the modified string there s_3
    let s_3 = s3.replace("dogs", "cats");    // <String>.replace(<string_part1>, <newPart>) is used to replace sub_string
    println!("{}", s_3);
    assert_eq!(s_3, "I like cats");

    // 4
    let s_1: String = String::from("hello,");
    let s_2: String = String::from("world!");
    let s_3: String = s_1 + &s_2;    // &String == &str string-slice, &str can only be concatenated to a String from its ending portion 
    assert_eq!(s_3, "hello,world!");
    println!("{}", s_3);      // Now s_1 is owned by s_3

    // 5
    let s4: &str = "String 4 is ree...!!";
    str_to_string_ji(s4.to_string());  // <<--- &str can be changed to String using this method..
    // str_to_string_ji(String::from(s4));  // <<--- &str can be changed to String using this method also
    
    // 6
    let s5 = "Ki Holo Bhaya".to_string();
    println!("String type -- {}", s5);
    let s_5 = &s5;      // Changed the String type to &str with a &
    // let s_5 = s5.as_str();      // Changed the String type to &str with  "<content>".as_str()
    println!("Changed to &str -- {}", s_5);

    // 7 
    // You can use escapes to write bytes by their hexadecimal values
    let byte_escape = "I'm writing Ru\x73\x74!";      // "I'm writing Rust"
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);    // "\\" in String to print out \

    // ...Or Unicode code points.
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!("Unicode character {} (U+211D) is called {}",
                unicode_codepoint, character_name );

    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here \
                         can be escaped too!";
    println!("{}", long_string);

    // 8 
    /* The r over here will make this string to print every character as it is written in the string. */
    let raw_str = r"Escapes work here using 'r': \x3F \u{211D}";  
    println!("{}", raw_str);
    let raw_str = "Escapes don't work here: \x3F \u{211D}";  
    assert_eq!(raw_str, "Escapes don't work here: ? ℝ");

    /* If you need quotes in a raw string, add a pair of #s */
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    /* If you need "# in your string, just use more #s in the delimiter. You can use up to 65535 #s. */
    let delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", delimiter);

    // 9 
    // We can't use indexing to access char in strings but can use slicing by &<String_var>[<start>..<end>] ; end := end - 1 
    let s1 = String::from("Oh hi Mork!!");
    let lt: &str = &s1[0..2]; 
    assert_eq!(lt, "Oh");

    let h1 = &s1[6..11];
    assert_eq!(h1, "Mork!");

    // 10 
    for el in "What's the dog doing?!?".chars() {
        println!("{}", el);
    }



    /*
    Slice -- It is a reference to contiguous sequence to a collection. 
    > It provides a way to borrow a part of a collection without taking ownership of the collection.
    > It can be created from Vectors, arrays, Strings and other colections implementing the 'Deref' trait.
    > Slices are similar to arrays, but their length is not known at compile time, so you can't use slice directly. 
    > It works like string slices by storing a reference to the first element and a length.
    */

    // Examples >> 1 
    let ar: [i32; 5] = [1, 2, 3, 4, 5];
    let slice = &ar[1..3];   // slice variable is aview to the given portion of this array
    assert_eq!(slice, &[2, 3]);

    // 2 
    // Here, both [i32] and str are slice types, but directly using it will cause errors. 
    let arr: [i32; 3] = [1, 2, 3];
    let s1: &[i32] = &arr[0..2];  // [1, 2]
    assert_eq!(s1, &arr[0..2]);
    let s2: &str = "hello, world";
    println!("{}", s2);

    // 3 
    let s = String::from("hello");
    let slice1: &str = &s[0..2];
    // DON'T USE 0..2 again
    let slice2: &str = &s[..2];
    assert_eq!(slice1, slice2);

    // 4 
    let mut s7 = String::from("hello world");
    // let s7 = String::from("hello world");
    /* Here, &s is `&String` type, but `first_letter` needs a `&str` type.
     It works because `&String` can be implicitly converted to `&str. If you want to know more, this is called `Deref coercion`. */
     let letter = first_letter(&s7);    // &String -> &str
     
     println!("the first letter is: {}", letter);
     s7.clear(); // It will show error, if we use it before using the variable s7



    // let mut inp = String::new();
    // io::stdin().read_line(&mut inp).expect("Line Reading error...");
    /* We are drawing a mutable reference to the variable 'inp', which helps read_line() method to directly make changes to the data in the variable. This read_line() functioin only copies the given input, so we used '&mut' here, which gives a mutable access to modify. */
    // println!("User-Input ==> {}", inp);



    /* Type-Casting:
      let <variable> = <value><datatype>;  // The new datatype to be assigned must stay beside the value.
      let <variable> = <value>_<datatype>;  // The _ separted the value from new datatype too.
      let <variable> = <value> as <new_datatype>;
    */
    let x = 321i64; // From i32 to i64
    let y = 128_i16; // From i32 to i16
    let w = 543_208i64; // An _ within the value also means the whole value, w holds value 543208
    let v = (w / x) + (y as i64);
    println!("{}", v);

    let k = (i32::MAX as i64) + 1;
    let j = 36000_i32;
    println!("This will give an overflow error , {}", k as i32 + j); // This will give an overflow error

    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("Expected a number input....");
    // let num_inp: i64 = inp.trim().parse().unwrap();     // Parsing the datatype in the variable side
    let num_inp = inp.trim().parse::<i64>().unwrap(); // Parsing the input via parse function
    println!("Number I/O , {}", num_inp);



    // Control Flow
    let cond1 = (2_f64) <= 2.2;
    println!("{}", cond1);
    // ! then && then ||
    let cond2 = cond1 || '@' == 'm' && !((56_i32) + (93_i32) >= (140_i32) || 'B' < 'r') && false;
    println!("{}", cond2);

    // formality man.....
    let fruit = "Apple";
    if fruit == "Banana" && fruit != "Shit" {
        println!("Give some bananas...");
    } else if fruit == "Apple" && fruit != "Shit" {
        println!("Give some apples...");
    } else if fruit == "Shit" {
        println!("Alien fuel....");
    } else {
        println!("Find someting else dawg....");
    }



    // Looping
    for i in 0..11 {
        print!("i:{} ; ", i);
    }
    println!(" | In for loop with start and end points, it will give uptil (endpoint - 1) and those two points separated by '..' ");

    // Infinite loop with 'loop'
    let mut c = 0;
    loop {
        c += 1;
        println!("Count : {}", c);
        // If there is no such break statement here, this loop woulld have ran till infinity...
        if c == 8 {
            break;
        }
    }

    // Conditional loop
    let ar: [i32; 6] = [1, 2, 3, 4, 5, 6];

    for el in ar.iter() {
        println!("Value : {}", el);
    }


    /*
    Owner :-- Owner of a value is the variable of the data structure that holds it and is responsible for allocating and freeing the memory used to store the data.

    Rules of ownership :--
    > Each value has an owner.
    > There can only be one owner at a time.
    > When the owner goes out of scope the value will be dropped.
     */
    // Example for ownership ---
    let t: (String, String) = (String::from("Kaisan"), String::from("Ho bhaa.."));
    let (s1, s2) = t.clone();
    println!("({:?} , {:?}) == t which have {:?}", s1, s2, t);



    /*
    Borrowing :-- Way of temporarily acessing data without taking ownership of it.
    > When borrowing, you're taking a reference (pointer), not the whole data itself.
    > Data can be borrowed mutable and immutably.

    Rules of borrowing / references :--
    > At a given time, you can have one mutable reference or any number of mutable references. References must be valid.
     */
    // Exampe of borrowing --
    // 1
    let sr = String::from("This is my string!!");
    let len = calc_length(&sr);                // This & represents referencing
    println!("Length of string '{}' = {}", sr, len);

    // 2
    let mut sr: String = String::from("hello there");
    println!("{}", sr);
    change(&mut sr);
    println!("{}", sr);
    
    // 3    ; If you want to have more than one mutable reference.
    let mut sr2: String = String::from("Another STRING..!!");
    println!("{}", sr2);

    {
        let ref1 = &mut sr2;   // ref1 goes out of main scope, so we can have this one mutable reference.
        println!("Reference 1 (out of main scope) => {}", ref1);
    }

    let ref2 = &mut sr2;
    println!("Reference 2 (in main scope) => {}", ref2);

    // 4    ; To either have a single mutable references and either number of immutable references
    let mut sr3 = String::from("One another STRING..!!!");

    let rf1 = &sr3;
    let rf2 = &sr3;
    println!("Immutable References => {} and {}", rf1, rf2);
    // Variables rf1 and rf2 will not be used after being used before or after the use of mutable reference.

    let rf3 = &mut sr3;
    println!("Single mutable reference => {}", rf3);

    // 5
    // let random_refrnc = dangle();
    let random_refrnc = no_dangle();
    println!("{}", random_refrnc);


    let nm: i32 = 56;
    let p: &i32 = &nm;
    println!("Memory address of variable nm is {:p}", p);   // {:<ref_var>} , here ':' will refer to the memory address of value which is being held by variable x.
    assert_eq!(56, *p);      // *p refers to dereferencing and showing the variable value it referenced
    println!("--------");
}




fn greet_re(s: &str) {
    println!("The greetings :->> {}", s);
}

fn str_to_string_ji(s: String) {
    println!("&str to String converted , {}", s);
}

fn calc_length(s: &String) -> usize {
  s.len()
}

fn change(str_inp: &mut String) {
    str_inp.push_str(", Mando !!");
}

fn first_letter(s: &str) -> &str {
    &s[..5]
}

fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
    (nums.0 + nums.1, nums.0 * nums.1)
}

/* 
fn dangle() -> &String {
    let s = String::from("Dangle waala string..");
    &s
}
*/

fn no_dangle() -> String {
    let s = String::from("No-Dangle waala string..");
    s
}
