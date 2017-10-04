use std;
use std::rc::Rc;
use std::any::Any;
use std::hash::Hash;
use std::path::PathBuf;
use std::collections::HashMap;

pub trait Tome {
    fn load(&self) -> Option<Rc<Asset>>;
    fn destroy(&self, page: Rc<Asset>);
}

pub trait Asset {}

struct TomeState {
    tome: Rc<Tome>,
    pages: Vec<Rc<Asset>>,
}

pub struct Librarian<T> {
    tomes: HashMap<T, Rc<TomeState>>,
}

impl<T: Eq + Hash> Librarian<T> {
    pub fn new() -> Librarian<T> {
        Librarian::<T> { tomes: HashMap::new() }
    }

    /*pub fn fetch(&mut self, tome_type: T) -> Option<Rc<Asset>> {
        self.tomes.get(&tome_type).and_then(
            |state| state.tome.load(),
        )
    }*/

    pub fn record() {}

    pub fn release() {}

    pub fn tome<H: Tome + 'static>(&mut self, tome_type: T, tome: H) {
        self.tomes.insert(
            tome_type,
            Rc::new(TomeState {
                tome: Rc::new(tome),
                pages: Vec::new(),
            }),
        );
    }
}

impl<T> Drop for Librarian<T> {
    fn drop(&mut self) {}
}
