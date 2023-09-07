use base64::{engine::general_purpose, Engine as _};

pub fn decode_base64(base64_content: &String) -> String {
    // Decodificar el contenido en base64
    let decoded_content = general_purpose::STANDARD.decode(base64_content).unwrap();
    // Convertir el contenido decodificado en una cadena UTF-8
    String::from_utf8_lossy(&decoded_content).to_string()
}
