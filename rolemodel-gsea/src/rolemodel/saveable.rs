pub trait Saveable {
    type Output;

    fn save(&mut self);
    fn restore(&mut self);
    fn current(&self) -> Self::Output;
    fn saved(&self) -> Self::Output;
}
pub trait RcSaveable {
    type T;

    fn save(self);
    fn restore(self);
    fn current(self) -> Self::T;
    fn saved(self) -> Self::T;
}

#[derive(Clone, Copy)]
pub struct SaveableData<T>
where T: Copy {
    current: T,
    saved: T,
}
impl<T> SaveableData<T> 
where T: Copy {
    fn new(current: T, saved: T) -> Self {
        Self {
            current,
            saved,
        }
    }
}
impl<T> Saveable for SaveableData<T> 
where T: Copy {
    type Output = T;

    fn save(&mut self) {
        self.saved = self.current.clone();
    }
    fn restore(&mut self) {
        self.current = self.saved.clone();
    }
    fn current(&self) -> Self::Output {
        self.current
    }
    fn saved(&self) -> Self::Output {
        self.saved
    }
}

impl<T> rv::data::Booleable for SaveableData<T> 
where 
    T: rv::data::Booleable 
{
    fn try_into_bool(self) -> Option<bool> {
        self.current.try_into_bool()
    }
    fn from_bool(b: bool) -> Self {
        Self::new(T::from_bool(b), T::from_bool(b))
    }
}