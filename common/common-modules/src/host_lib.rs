use common_types::PortId;

multiversx_sc::imports!();

/*
 /**
     * @dev validatePortIdentifier validates a port identifier string
     * check if the string consist of characters in one of the following categories only:
     * - Alphanumeric
     * - `.`, `_`, `+`, `-`, `#`
     * - `[`, `]`, `<`, `>`
     *
     * The validation is based on the ibc-go implementation:
     * https://github.com/cosmos/ibc-go/blob/b0ed0b412ea75e66091cc02746c95f9e6cf4445d/modules/core/24-host/validate.go#L86
     */
    function validatePortIdentifier(bytes memory portId) internal pure returns (bool) {
        uint256 portIdLength = portId.length;
        if (portIdLength < 2 || portIdLength > 128) {
            return false;
        }
        unchecked {
            for (uint256 i = 0; i < portIdLength; i++) {
                uint256 c = uint256(uint8(portId[i]));
                // return false if the character is not in one of the following categories:
                // a-z
                // 0-9
                // A-Z
                // ".", "_", "+", "-"
                // "#", "[", "]", "<", ">"
                if (
                    !(c >= 0x61 && c <= 0x7A) && !(c >= 0x30 && c <= 0x39) && !(c >= 0x41 && c <= 0x5A)
                        && !(c == 0x2E || c == 0x5F || c == 0x2B || c == 0x2D)
                        && !(c == 0x23 || c == 0x5B || c == 0x5D || c == 0x3C || c == 0x3E)
                ) {
                    return false;
                }
            }
        }
        return true;
    }

*/

const MIN_PORT_LEN: usize = 2;
const MAX_PORT_LEN: usize = 128;
const SPECIAL_CHARS: &[u8] = b"._+-#[]<>";

#[multiversx_sc::module]
pub trait HostLibModule {
    /// check if the string consist of characters in one of the following categories only:
    ///
    /// - Alphanumeric
    ///
    /// - `.`, `_`, `+`, `-`, `#`
    ///
    /// - `[`, `]`, `<`, `>`
    fn is_valid_port_id(&self, port_id: &PortId<Self::Api>) -> bool {
        let port_len = port_id.len();

        // Clippy suggestion makes code unreadable
        #[allow(clippy::manual_range_contains)]
        if port_len < MIN_PORT_LEN || port_len > MAX_PORT_LEN {
            return false;
        }

        let mut as_array = [0; MAX_PORT_LEN];
        let slice = port_id.load_to_byte_array(&mut as_array);
        for character in slice {
            if !character.is_ascii_alphanumeric() && !SPECIAL_CHARS.contains(character) {
                return false;
            }
        }

        true
    }
}
