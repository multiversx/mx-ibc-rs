multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait CheckCharModule {
    #[inline]
    fn is_ascii(&self, character: u8) -> bool {
        character >= b'a' && character <= b'z'
    }

    #[inline]
    fn is_big_ascii(&self, character: u8) -> bool {
        character >= b'A' && character <= b'Z'
    }

    #[inline]
    fn is_numeric(&self, character: u8) -> bool {
        character >= b'0' && character <= b'9'
    }

    fn is_alphanumeric(&self, character: u8) -> bool {
        self.is_ascii(character) || self.is_big_ascii(character) || self.is_numeric(character)
    }

    #[inline]
    fn is_hyphen(&self, character: u8) -> bool {
        character == b'-'
    }
}
