fn foo(input: &mut i32, times: &mut i32) -> bool
{
    *input *= *times;
    println!("{}", format!(" = {}", *input));

    if *input <= 0
    {
        *input = -99;
        return false;
    }
    
    *times += 1;
    // *input = if *input >= 10000 { 0 } else { *input };

    true
}

fn main()
{    
    let (mut input, mut times, mut ret_flag) = (0, 1, true);
    // let (a, b) = (&input, &times);
    while ret_flag && times <= 12 
    {
        print!("{}", format!("Calculation: {} * {}", input, times));
        ret_flag = foo(&mut input, &mut times);
    }

    println!("--> {}", format!("Terminated without error - {ret_flag}"));

    // assert_eq!(foo(foo(3)), 100);

    println!("Hello, world!");
}


/*
Rust omits keyword 'return'. However, it is required on escaping.
Rust has no increment(++) or decrement(--) operator
Rust has no ternary operator
Difference in argument usage: Pass by Reference 
*/