pub trait Sort<T: PartialOrd> {
    fn sort(&self, input: &mut Vec<T>);
}