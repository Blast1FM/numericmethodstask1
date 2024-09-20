use calc::FunctionIterator;

fn main() {
    
    let calculator = FunctionIterator::new((0.5,1.0), 30, Box::new(|y:f64| 4.0/y ));
    let step:f64 = calculator.delta;
    let result:f64 = calculator.map(|y| y * &step).sum();

    println!("Result: {}, Delta: {}", result, step);
}
