use base64_decoder_cli::decode_base64::decode_base64;

#[test]
fn decode_text() {
    let texto_base64 = String::from("SGVsbG8gd29ybGQh");
    let result = decode_base64(&texto_base64);
    assert_eq!(result, String::from("Hello world!"));
}
