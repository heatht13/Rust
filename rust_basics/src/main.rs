use std::collections::HashMap;
fn main() {

    //DATA TYPES
    //CONSTS
    const THIS_IS_A_CONST: i32 = 58;
    println!{"Const: {}", THIS_IS_A_CONST}

    //use 'mut' to allow modification/mutation of variables

    //NUMBERS
    let uint: u32 = 35;
    let sint: i16 = 57;
    println!("Numbers:\n - Unsigned Integer: {}\n - Signed Integer: {}", uint, sint);
    
    let flt: f64 = 45.3843823;
    println!(" - Float: {}", flt);

    //STRINGS-STRING REFS
    let str_obj: String = String::from("instance of String");
    let str_ref: &str = "ref to a string";
    println!("Strings:\n - Instance of String: {}\n - Reference: {}", str_obj, str_ref);

    //CHARS
    let chr: char = 'h';
    println!("Chars: {}", chr);

    //BOOLEANS
    let bol: bool = false;
    println!("Booleans: {}", bol);

    //TUPLES
    let tup: (&str, u32, bool) = ("string", 17, true);
    println!("Tuple: {:?}", tup);

    //ARRAY REFERENCES-SLICES zero based indexing
    let arr: [u32; 5] = [1,2,3,4,5];
    let slc: &[u32] = &arr[1..3]; //dynamic size, reference to data in arr
    println!("Arrays:\n - Reference: {:?}\n - Slice: {:?}", arr, slc);

    //VECTORS-DYNAMIC ARRAYS zero based indexing
    let mut vect: Vec<u32> = vec![45, 64, 1]; //or Vec::new() or Vec::from([])
    println!("Vectors:\n - Initial: {:?}", vect);
    vect.push(17);
    println!(" - Push: {:?}", vect);
    vect.push(87);
    println!(" - Push: {:?}", vect);
    vect.remove(0);
    println!(" - Remove elem 0: {:?}", vect);

    //use 'pub' to designate as public. Defaults to private if not specified

    //STRUCT
    #[derive(Debug)]
    struct Crypto{
        pub coin: String,
        is_nice: bool
    }
    impl Crypto{
        pub fn update_nice(&mut self, nice: bool) -> bool {
            self.is_nice = nice;
            self.is_nice
        }
    }
    let coin_name: String = String::from("SOL");
    let mut coin: Crypto = Crypto{coin: coin_name, is_nice: false};
    coin.update_nice(true);
    println!("Struct: {:#?}", coin); //{:#?} = pretty print
    
    //ENUM
    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6
    }
    let four: IpAddrKind = IpAddrKind::V4;
    println!("Enum: {:?}", four);

    //HASHMAPS
    let mut dictionary: HashMap<char, u32> = HashMap::new();
    dictionary.insert('a', 0);
    dictionary.insert('b', 1);
    dictionary.insert('c', 2);
    dictionary.insert('d', 3);
    println!("HashMap: {:#?}", dictionary);

    //OPTIONS
    match dictionary.remove(&'b') {
        Some(val) => {println!("Option: Value is {}", val);},
        None => {println!("Option: Could not remove value with key b");}
    }

    //FUNCTIONS
    pub fn add_positive(a: u32, b: u32) -> Result<u32, &'static str> {
        let c:u32 = a+b;
        if c >= 0 {
            Ok(c)
        }
        else {
            Err("The summation fo arguments is not positve.")
        }
    }

    //RESULT
    match add_positive(0, 16){
        Ok(c) => {println!("Result: {}", c)},
        Err(val) => {println!("Result: {}", val);}
    }

    // ---------------------------------------------------------------------------------------------------------------------------------
    //FLOW CONTROL
    //IF STATEMENT
    println!("If Statement:");
    if chr == 'h' {
        println!("Yes, it is h");
    } else if chr == 'c' {
        println!("No, its c");
    } else {
        println!("No, its value is: {}", chr);
    }

    //MATCH STATEMENT
    println!("Match Statement:");
    match four {
        IpAddrKind::V4 => {println!("It is V4");},
        IpAddrKind::V6 => {println!("It is V6");}
    }

    //Loop
    let mut loop_var: u32 = 1;
    println!("Loop:");
    loop {
        println!("Iteration: {}", loop_var);
        if loop_var > 3 {
            break
        }
        loop_var += 1;
    }

    //FOR LOOP
    println!("For Loop:");
    for element in 0..vect.len(){
        println!("Iteration {}: {}", element, vect[element]);
    }

    //WHILE LOOP
    println!("While Loop:");
    loop_var = 1;
    while loop_var < 4 {
        println!("Iteration: {}", loop_var);
        loop_var += 1;
    }
}