use std::fmt;

pub struct User {
    nombre: String,
}

impl User {
    pub fn new(nombre: String) -> Self {
        Self { nombre }
    }
}

impl Default for User {
    fn default() -> Self {
        Self {
            nombre: "desconocido".to_owned(),
        }
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.nombre)
    }
}
