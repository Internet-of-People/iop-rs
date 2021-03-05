use super::*;

pub trait State<T: 'static + Send + Sync>: Send + Sync {
    fn clone(&self) -> Box<dyn State<T>>;
    fn try_borrow(&self) -> Result<MappedRwLockReadGuard<'_, T>>;
    fn try_borrow_mut(&mut self) -> Result<MappedRwLockWriteGuard<'_, T>>;
}

impl<T: 'static + Send + Sync> dyn State<T> {
    pub fn map<R: 'static + Send + Sync>(
        &self, adapt: fn(&T) -> &R, adapt_mut: fn(&mut T) -> &mut R,
    ) -> Box<dyn State<R>> {
        let state = self.clone();
        let adapter = StateAdapter { state, adapt, adapt_mut };
        Box::new(adapter)
    }
}

impl<T: 'static + Sync + Send> State<T> for Arc<RwLock<T>> {
    fn clone(&self) -> Box<dyn State<T>> {
        Box::new(self.to_owned())
    }

    // TODO all try_read and try_write calls should block instead with read_until() and write_until()
    fn try_borrow(&self) -> Result<MappedRwLockReadGuard<'_, T>> {
        let r = self.try_read().ok_or_else(|| format_err!("Read lock on Vault state failed"))?;
        Ok(RwLockReadGuard::map(r, |x| x))
    }

    fn try_borrow_mut(&mut self) -> Result<MappedRwLockWriteGuard<'_, T>> {
        let w = self.try_write().ok_or_else(|| format_err!("Write lock on Vault state failed"))?;
        Ok(RwLockWriteGuard::map(w, |x| x))
    }
}

struct StateAdapter<T: 'static, R: 'static> {
    state: Box<dyn State<T>>,
    adapt: fn(&T) -> &R,
    adapt_mut: fn(&mut T) -> &mut R,
}

impl<T: 'static + Send + Sync, R: 'static + Send + Sync> State<R> for StateAdapter<T, R> {
    fn clone(&self) -> Box<dyn State<R>> {
        let state = self.state.as_ref().clone();
        let adapt = self.adapt;
        let adapt_mut = self.adapt_mut;
        Box::new(StateAdapter { state, adapt, adapt_mut })
    }

    fn try_borrow(&self) -> Result<MappedRwLockReadGuard<'_, R>> {
        let r = self.state.try_borrow()?;
        Ok(MappedRwLockReadGuard::map(r, self.adapt))
    }

    fn try_borrow_mut(&mut self) -> Result<MappedRwLockWriteGuard<'_, R>> {
        let w = self.state.try_borrow_mut()?;
        Ok(MappedRwLockWriteGuard::map(w, self.adapt_mut))
    }
}
