use common_types2::PortId;

const MIN_PORT_LEN: usize = 2;
const MAX_PORT_LEN: usize = 128;
const SPECIAL_CHARS: &[u8] = b"._+-#[]<>";

/// check if the string consist of characters in one of the following categories only:
///
/// - Alphanumeric
///
/// - `.`, `_`, `+`, `-`, `#`
///
/// - `[`, `]`, `<`, `>`
pub fn is_valid_port_id(port_id: &PortId) -> bool {
    let port_len = port_id.len();

    // Clippy suggestion makes code unreadable
    #[allow(clippy::manual_range_contains)]
    if port_len < MIN_PORT_LEN || port_len > MAX_PORT_LEN {
        return false;
    }

    for character in port_id {
        if !character.is_ascii_alphanumeric() && !SPECIAL_CHARS.contains(character) {
            return false;
        }
    }

    true
}
