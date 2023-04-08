
// https://stackoverflow.com/questions/65203307/how-do-i-create-a-trait-object-that-implements-fn-and-can-be-cloned-to-distinct
pub (crate) trait CopyableFn<'a, T:'a,const D: usize>: Fn(&[T;D]) -> T + BoxFnClone<'a,T,D> {}

impl<'a,T, const D: usize> CopyableFn<'a, T,D> for T where T: Fn(&[T;D]) -> T  + Clone + 'static{}

pub (crate) trait BoxFnClone<'a, T:'a, const D: usize> {
    fn clone_box(&self) -> Box<dyn CopyableFn<'a,T,D>>;
}

impl<'a, T, const D:usize> BoxFnClone<'a,T,D> for T
where
    T: 'static + CopyableFn<'a,T,D> + Clone,
{
    fn clone_box(&self) -> Box<dyn CopyableFn<'a,T,D>> {
        Box::new(self.clone())
    }
}

impl<'a,T:'a,const D: usize> Clone for Box<dyn CopyableFn<'a,T,D>> {
    fn clone(&self) -> Box<dyn CopyableFn<'a, T,D>> {
        (**self).clone_box()
    }
}