use std::fs::{read, write};
use std::io::Cursor;
use std::path::Path;

use tfhe::safe_serialization::{safe_deserialize, safe_serialize};
use tfhe::shortint::parameters::v1_2::V1_2_PARAM_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M64;
use tfhe::{generate_keys, set_server_key, ClientKey, ConfigBuilder, ServerKey};

const PATH_CLIENT: &str = ".tfhe/client.key";
const PATH_SERVER: &str = ".tfhe/server.key";

pub fn initialize_keys() -> ClientKey {
    let client_key: ClientKey;
    let server_key: ServerKey;

    if Path::new(PATH_CLIENT).exists() && Path::new(PATH_SERVER).exists() {
        let client_key_buffer = read(PATH_CLIENT).unwrap();
        let client_key_cursor = Cursor::new(client_key_buffer);

        client_key = safe_deserialize(client_key_cursor, u64::MAX).unwrap();

        let server_key_buffer = read(PATH_SERVER).unwrap();
        let server_key_cursor = Cursor::new(server_key_buffer);

        server_key = safe_deserialize(server_key_cursor, u64::MAX).unwrap();
    } else {
        (client_key, server_key) = generate_keys(
            ConfigBuilder::with_custom_parameters(
                V1_2_PARAM_MESSAGE_2_CARRY_2_KS_PBS_TUNIFORM_2M64,
            )
            .build(),
        );

        let mut client_key_buffer = Vec::new();
        let mut client_key_cursor = Cursor::new(&mut client_key_buffer);

        safe_serialize(&client_key, &mut client_key_cursor, u64::MAX).unwrap();
        write(PATH_CLIENT, client_key_buffer).unwrap();

        let mut server_key_buffer = Vec::new();
        let mut server_key_cursor = Cursor::new(&mut server_key_buffer);

        safe_serialize(&server_key, &mut server_key_cursor, u64::MAX).unwrap();
        write(PATH_SERVER, server_key_buffer).unwrap();
    }

    set_server_key(server_key);

    client_key
}
