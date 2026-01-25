#[test]
fn test_derive() {
    use print_name::PrintName;
    use print_name_macros::PrintName;

    #[derive(PrintName)]
    struct MyStruct;

    assert_eq!(MyStruct::name(), "MyStruct");
}
