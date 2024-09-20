use std::cmp::Ordering;
pub struct FunctionIterator
{
    pub delta: f64,
    current_x : f64,
    function : Box<dyn Fn(f64) -> f64>,
    limits : (f64,f64)
}

impl FunctionIterator
{
    pub fn new(limits:(f64,f64), steps:i32, function:Box<dyn Fn(f64)->f64>) -> Self
    {
        Self
        {
            delta: (limits.1 - limits.0)/f64::from(steps),
            current_x: limits.0,
            function,
            limits
        }
    }
}

impl Iterator for FunctionIterator
{
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item>
    {
        match self.current_x.total_cmp(&self.limits.1)
        {
            Ordering::Less => 
            {
                let func_value = (self.function)(self.current_x);
                self.current_x += self.delta;
                Some(func_value)
            }
            _ => None,
        }
    }
}