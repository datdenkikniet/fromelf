#[test]
fn perform_copy() {
    let bytes = include_bytes!("./example.elf");
    let arm_none_eabi_bin = include_bytes!("./example.bin");

    let converted = fromelf::elf_bytes_to_bin(bytes.as_ref()).unwrap();
    assert_eq!(&converted, arm_none_eabi_bin.as_ref());
}
