fn main() {
    let y;
    {
        let x = vec!['a','b','c'];
        y = &x; // Try just 'x' to move ownership to y.
                // Or try 'x.clone()' to completely duplicate x.
        println!("{}", x[0]);
    }
    println!("{}", y[0]);
}
