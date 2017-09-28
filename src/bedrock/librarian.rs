use std;
use std::any::Any;
use std::hash::Hash;
use std::path::PathBuf;
use std::collections::HashMap;

pub trait Tome {
    fn load(&mut self) -> Result<Box<Any>, bool>;
    fn destroy(&mut self, page: Box<Any>);
}

struct TomeState {
    tome: Box<Tome>,
    pages: Vec<Box<Any>>,
}

pub struct Librarian<T> {
    tomes: HashMap<T, TomeState>,
}

impl<T: Eq + Hash> Librarian<T> {
    pub fn new() -> Librarian<T> {
        Librarian::<T>{
            tomes: HashMap::new(),
        }
    }

    pub fn fetch(&mut self, tome_type: T) -> Option<&Box<Any>> {
        let mut tome_state = self.tomes.get_mut(&tome_type).unwrap();

        let page = tome_state.tome.load().unwrap();
        tome_state.pages.push(page);

        Some(tome_state.pages.last().unwrap())
    }

    pub fn record() {
    }

    pub fn release() {
    }

    pub fn tome<H: Tome+'static>(&mut self, tome_type: T, tome: H) {
        self.tomes.insert(tome_type, TomeState{
            tome: Box::new(tome),
            pages: Vec::new(),
        });
    }
}

impl<T> Drop for Librarian<T> {
    fn drop(&mut self) {
    }
}
