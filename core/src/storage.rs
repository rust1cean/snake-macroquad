use std::{
    any::{type_name, Any},
    collections::{hash_map::DefaultHasher, HashMap},
    fmt::Debug,
    hash::{Hash, Hasher},
};

pub type Token = u64;
pub type Item = Box<dyn Component>;
pub type Items = Vec<Item>;

// TODO: Add macros for 'as_any' and 'as_any_mut' methods
pub trait Component: Debug + Send + 'static {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct Storage {
    storage: HashMap<Token, Items>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }

    pub fn add<T: Component>(&mut self, component: T) -> &mut Self {
        // Create unique token from component type
        let token: Token = Self::type_to_token::<T>();

        let component: Box<T> = Box::new(component);

        // Check if it exists
        self.storage.entry(token).or_insert(vec![]);

        // Push component
        self.storage
            .entry(token)
            .and_modify(|components| components.push(component));

        self
    }

    pub fn get_all<T: Component>(&self) -> Option<Vec<&T>> {
        let token: Token = Self::type_to_token::<T>();

        if let Some(components) = self.storage.get(&token) {
            return Some(
                components
                    .iter()
                    .map(component_as_type::<T>)
                    .collect::<Vec<_>>(),
            );
        }
        None
    }

    pub fn get_all_mut<T: Component>(&mut self) -> Option<Vec<&mut T>> {
        let token: Token = Self::type_to_token::<T>();

        if let Some(components) = self.storage.get_mut(&token) {
            return Some(
                components
                    .iter_mut()
                    .map(component_as_mut_type::<T>)
                    .collect::<Vec<_>>(),
            );
        }
        None
    }

    pub fn get_several_mut<T: Component, U: Component>(
        &mut self,
    ) -> Option<(Vec<&mut T>, Vec<&mut U>)> {
        let f_token: Token = Self::type_to_token::<T>();
        let s_token: Token = Self::type_to_token::<U>();

        if let Some([first, second]) = self.storage.get_many_mut([&f_token, &s_token]) {
            return Some((
                first
                    .iter_mut()
                    .map(component_as_mut_type::<T>)
                    .collect::<Vec<_>>(),
                second
                    .iter_mut()
                    .map(component_as_mut_type::<U>)
                    .collect::<Vec<_>>(),
            ));
        }
        None
    }

    pub fn get<T: Component>(&self, index: usize) -> Option<&T> {
        if let Some(mut components) = self.get_all::<T>() {
            return Some(components.remove(index));
        }
        None
    }

    pub fn get_mut<T: Component>(&mut self, index: usize) -> Option<&mut T> {
        if let Some(mut components) = self.get_all_mut::<T>() {
            return Some(components.remove(index));
        }
        None
    }

    pub fn get_first<T: Component>(&self) -> Option<&T> {
        self.get::<T>(0)
    }

    pub fn get_first_mut<T: Component>(&mut self) -> Option<&mut T> {
        self.get_mut::<T>(0)
    }

    pub fn remove<T: Component>(&mut self, index: usize) -> &mut Storage {
        // Create unique token from component type
        let token: Token = Self::type_to_token::<T>();

        // Remove component
        self.storage.entry(token).and_modify(|components| {
            components.remove(index);
        });

        self
    }
}

impl Identification for Storage {}

pub trait Identification {
    fn gen_token_from<T: Hash>(value: T) -> u64 {
        let mut s = DefaultHasher::new();
        value.hash(&mut s);
        s.finish()
    }

    fn type_to_token<T>() -> u64 {
        let name: &str = type_name::<T>();
        Self::gen_token_from(name)
    }
}

pub fn component_as_type<T: Component>(component: &Box<dyn Component>) -> &T {
    component.as_any().downcast_ref::<T>().unwrap()
}

pub fn component_as_mut_type<T: Component>(component: &mut Box<dyn Component>) -> &mut T {
    component.as_any_mut().downcast_mut::<T>().unwrap()
}
