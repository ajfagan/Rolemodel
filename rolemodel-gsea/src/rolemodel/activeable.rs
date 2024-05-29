
pub trait Activeable {
    fn is_active(&self) -> bool;
    fn set_activity(&mut self, b: bool); 

    fn switch_activity(&mut self) { self.set_activity( self.is_inactive() ) }
    fn is_inactive(&self) -> bool { !self.is_active() }
    fn is_legal(&self) -> bool;
    
    fn is_illegal(&self) -> bool { !self.is_legal() }
}
