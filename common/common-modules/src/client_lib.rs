use common_types::ClientId;

multiversx_sc::imports!();

// TODO: Change this if needed
const MAX_CLIENT_ID_LEN: usize = 128;

#[multiversx_sc::module]
pub trait ClientLibModule {
    /// client_id must be non-empty and max length of MAX_CLIENT_ID_LEN
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
        if len == 0 || len > MAX_CLIENT_ID_LEN {
            return false;
        }

        let mut as_array = [0u8; MAX_CLIENT_ID_LEN];
        let slice = client_id.load_to_byte_array(&mut as_array);
        let first_char = slice[0];
        if !first_char.is_ascii_lowercase() {
            return false;
        }

        let last_char = slice[len - 1];
        if !last_char.is_ascii_lowercase() && !last_char.is_ascii_digit() {
            return false;
        }

        // clippy suggestion is dumb
        #[allow(clippy::needless_range_loop)]
        for i in 1..len - 1 {
            let character = slice[i];
            if !character.is_ascii_lowercase() && !character.is_ascii_digit() && character != b'-' {
                return false;
            }
        }

        true
    }
}
