pub trait URTypeChar {
    fn is_ur_type(&self) -> bool;
}

impl URTypeChar for char {
    fn is_ur_type(&self) -> bool {
        if 'a' <= *self && *self <= 'z' {
            return true;
        }
        if '0' <= *self && *self <= '9' {
            return true;
        }
        if *self == '-' {
            return true;
        }
        false
    }
}

pub trait URTypeString {
    fn is_ur_type(&self) -> bool;
}

impl URTypeString for &str {
    fn is_ur_type(&self) -> bool {
        self.chars().all(|c| c.is_ur_type())
    }
}

impl URTypeString for String {
    fn is_ur_type(&self) -> bool {
        self.as_str().is_ur_type()
    }
}
