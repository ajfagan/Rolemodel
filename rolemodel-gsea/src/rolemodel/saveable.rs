use rv::data::Booleable;

use crate::Activeable;

pub trait Saveable {
    type Output;

    fn save(&mut self);
    fn restore(&mut self);
    fn current(&self) -> Self::Output;
    fn saved(&self) -> Self::Output;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SaveableData<T>
where T: Copy {
    current: T,
    saved: T,
}
impl<T> SaveableData<T> 
where T: Copy {
    pub fn new(current: T, saved: T) -> Self {
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
        self.saved = self.current;
    }
    fn restore(&mut self) {
        self.current = self.saved;
    }
    fn current(&self) -> Self::Output {
        self.current
    }
    fn saved(&self) -> Self::Output {
        self.saved
    }
}
impl<T> Activeable for SaveableData<T>
where
    T: Copy + Booleable
{
    fn is_active(&self) -> bool {
        self.current().into_bool()
    }

    fn set_activity(&mut self, b: bool) {
        self.save();
        self.current = T::from_bool(b);
    }

    fn is_legal(&self) -> bool {
        true
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