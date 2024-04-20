use serde::{Deserialize, Serialize};

use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub nombre: String,
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

impl From<String> for User {
    fn from(value: String) -> Self {
        User { nombre: value }
    }
}

impl From<(&str, &str)> for User {
    fn from(value: (&str, &str)) -> Self {
        let mut n0: String = value.0.to_owned();
        let n1: &str = value.1;

        n0.push_str(&n1);

        User { nombre: n0 }
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.nombre)
    }
}

#[cfg(test)]
mod tests {
    use super::User;

    #[test]
    fn test_from_string() {
        let user_david = User::from("david".to_owned());
        assert_eq!(
            user_david.nombre,
            User {
                nombre: "david".to_owned()
            }
            .nombre
        );
    }

    #[test]
    fn test_from_default() {
        let user_david = User::default();

        assert_eq!(
            user_david.nombre,
            User {
                nombre: "desconocido".to_owned()
            }
            .nombre
        );
    }

    #[test]
    fn test_from_tuple() {
        let tuplas: (&str, &str) = ("David", "López");
        let user_david = User::from(tuplas);

        assert_eq!(
            user_david.nombre,
            User {
                nombre: "DavidLópez".to_owned()
            }
            .nombre
        );
    }
}
