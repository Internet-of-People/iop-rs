use super::*;

pub trait State<T: 'static> {
    fn clone(&self) -> Box<dyn State<T>>;
    fn try_borrow(&self) -> Fallible<Ref<'_, T>>;
    fn try_borrow_mut(&mut self) -> Fallible<RefMut<'_, T>>;
}

impl<T: 'static> dyn State<T> {
    pub fn map<R: 'static>(
        &self, adapt: fn(&T) -> &R, adapt_mut: fn(&mut T) -> &mut R,
    ) -> Box<dyn State<R>> {
        let state = self.clone();
        let adapter = StateAdapter { state, adapt, adapt_mut };
        Box::new(adapter)
    }
}

impl<T: 'static> State<T> for Rc<RefCell<T>> {
    fn clone(&self) -> Box<dyn State<T>> {
        Box::new(self.to_owned())
    }

    fn try_borrow(&self) -> Fallible<Ref<'_, T>> {
        let r = RefCell::try_borrow(self)?;
        Ok(r)
    }

    fn try_borrow_mut(&mut self) -> Fallible<RefMut<'_, T>> {
        let r = RefCell::try_borrow_mut(self)?;
        Ok(r)
    }
}

struct StateAdapter<T: 'static, R: 'static> {
    state: Box<dyn State<T>>,
    adapt: fn(&T) -> &R,
    adapt_mut: fn(&mut T) -> &mut R,
}

impl<T: 'static, R: 'static> State<R> for StateAdapter<T, R> {
    fn clone(&self) -> Box<dyn State<R>> {
        let state = self.state.as_ref().clone();
        let adapt = self.adapt;
        let adapt_mut = self.adapt_mut;
        Box::new(StateAdapter { state, adapt, adapt_mut })
    }

    fn try_borrow(&self) -> Fallible<Ref<'_, R>> {
        let r = Ref::map(self.state.try_borrow()?, self.adapt);
        Ok(r)
    }

    fn try_borrow_mut(&mut self) -> Fallible<RefMut<'_, R>> {
        let r = RefMut::map(self.state.try_borrow_mut()?, self.adapt_mut);
        Ok(r)
    }
}
