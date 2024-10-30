use common_types2::ClientType;

// TODO: Change this if needed
const MAX_CLIENT_TYPE_LEN: usize = 128;

pub fn is_valid_client_type(client_type: &ClientType) -> bool {
    let len = client_type.len();
    if len == 0 || len > MAX_CLIENT_TYPE_LEN {
        return false;
    }

    let first_char = client_type[0];
    if !first_char.is_ascii_lowercase() {
        return false;
    }

    let last_char = client_type[len - 1];
    if !last_char.is_ascii_lowercase() && !last_char.is_ascii_digit() {
        return false;
    }

    // clippy suggestion is dumb
    #[allow(clippy::needless_range_loop)]
    for i in 1..len - 1 {
        let character = client_type[i];
        if !character.is_ascii_lowercase() && !character.is_ascii_digit() && character != b'-' {
            return false;
        }
    }

    true
}
