// A Fibonacci sequence Function

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}

// Initialize a vector with 10 elements

let mut v = Vec::with_capacity(10);

// Push 10 elements into the vector

for i in 0..10 {
    v.push(fibonacci(i));
}

// Print the vector

println!("{:?}", v);