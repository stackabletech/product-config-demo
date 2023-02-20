use std::{collections::HashMap, marker::PhantomData};

use serde::Serialize;

pub trait Versioned {
    fn version() -> &'static str;
}

pub trait Downgrade<To> {
    fn downgrade(self) -> To;
}
impl<T> Downgrade<T> for T {
    fn downgrade(self) -> Self {
        self
    }
}

pub struct Versioner<T> {
    versions: HashMap<&'static str, fn(T) -> Box<dyn erased_serde::Serialize>>,
}

impl<T> Versioner<T> {
    pub fn new() -> Self
    where
        T: Versioned + Serialize + 'static,
    {
        Self {
            versions: HashMap::new(),
        }
        .with_old_version::<T>()
    }

    pub fn with_old_version<Old>(mut self) -> Self
    where
        Old: Versioned + Serialize + 'static,
        T: Downgrade<Old>,
    {
        self.versions.insert(Old::version(), |obj| {
            Box::new(obj.downgrade()) as Box<dyn erased_serde::Serialize>
        });
        self
    }

    pub fn downgrade_to(&self, version: &str, obj: T) -> Box<dyn erased_serde::Serialize> {
        let Some(downgrader) = self.versions.get(version) else {
            panic!("format version {version:?} not registered")
        };
        (downgrader)(obj)
    }
}
