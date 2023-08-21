use std::{
    fmt::{Debug, Display, Formatter},
    sync::Arc,
};
pub struct ControlFunction {
    pub escape: u8,
    pub params: [u8; 4],
}

pub struct ANSIChar {
    pub control: Arc<ControlSequence>,
    pub char: char,
}

pub type ControlSequence = Vec<ControlFunction>;
pub type ANSIString = Vec<ANSIChar>;

impl PartialEq for ANSIChar {
    fn eq(&self, other: &Self) -> bool {
        if self.char != other.char {
            return false;
        }

        let mut result = true;
        for (i, param) in self.control.iter().enumerate() {
            if param.escape != other.control[i].escape {
                result = false;
                break;
            }
            for (j, byte) in param.params.iter().enumerate() {
                if byte != &other.control[i].params[j] {
                    result = false;
                    break;
                }
            }
        }

        result
    }
}

impl Clone for ControlFunction {
    fn clone(&self) -> Self {
        ControlFunction {
            escape: self.escape.clone(),
            params: self.params.clone(),
        }
    }
}

impl Debug for ControlFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ControlFunction")
            .field("escape", &self.escape)
            .field("params", &self.params)
            .finish()
    }
}

impl Debug for ANSIChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ANSIChar")
            .field("control", &self.control)
            .field("char", &self.char)
            .finish()
    }
}

impl Display for ANSIChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::from("\x1B[");
        self.control.iter().for_each(|ctrl| {
            result.push_str(&format!("{}", ctrl));
        });
        result.push('m');
        result.push(self.char);
        write!(f, "{}", result)
    }
}

impl Display for ControlFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = format!("{};", self.escape);
        let max = result.len() - 1;
        self.params.iter().enumerate().for_each(|(i, param)| {
            result.push_str(&format!("{}", param));
            if i != max {
                result.push(';');
            }
        });
        write!(f, "{}", result)
    }
}
