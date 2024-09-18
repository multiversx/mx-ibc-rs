use common_types::ClientId;

multiversx_sc::imports!();

// TODO: Change this if needed
const MAX_CLIENT_TYPE_LEN: usize = 128;

#[multiversx_sc::module]
pub trait ClientLibModule: crate::check_char::CheckCharModule {
    /// client_id must be non-empty and max length of MAX_CLIENT_TYPE_LEN
    ///
    /// client_id must be in the form of `^[a-z][a-z0-9-]*[a-z0-9]$`
    ///
    /// i.e. Must start with a-z
    ///
    /// contains any of a-z, 0-9 or -
    ///
    /// ends with a-z or 0-9
    fn is_valid_client_id(&self, client_id: &ClientId<Self::Api>) -> bool {
        let len = client_id.len();
        if len == 0 || len > MAX_CLIENT_TYPE_LEN {
            return false;
        }

        let mut as_array = [0u8; MAX_CLIENT_TYPE_LEN];
        let slice = client_id.load_to_byte_array(&mut as_array);
        let first_char = slice[0];
        if !self.is_ascii(first_char) {
            return false;
        }

        let last_char = slice[len - 1];
        if !self.is_ascii(last_char) && !self.is_numeric(last_char) {
            return false;
        }

        for i in 1..len - 1 {
            let character = slice[i];
            if !self.is_ascii(character)
                && !self.is_numeric(character)
                && !self.is_hyphen(character)
            {
                return false;
            }
        }

        true
    }
}
