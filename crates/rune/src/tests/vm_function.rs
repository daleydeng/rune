prelude!();

use crate::runtime::SignedType;

#[test]
fn test_function() -> Result<()> {
    let context = Arc::try_new(Context::with_default_modules()?)?;

    // ptr to dynamic function.
    let function: Function = rune! {
        fn foo(a, b) { a + b }
        foo
    };

    assert_eq!(function.call::<SignedType>((1, 3)).unwrap(), 4);
    assert!(function.call::<SignedType>((1,)).is_err());

    // ptr to native function
    let function: Function = rune!(Vec::new);

    let value: Vec<Value> = function.call(()).unwrap();
    assert_eq!(value.len(), 0);

    // ptr to dynamic function.
    let function: Function = rune! {
        enum Custom { A(a) }
        Custom::A
    };

    assert!(function.call::<Value>(()).is_err());
    let value: Value = function.call((1,)).unwrap();
    assert!(rune::from_value::<DynamicTuple>(value).is_ok());

    // ptr to dynamic function.
    let function: Function = rune! {
        struct Custom(a);
        Custom
    };

    assert!(function.call::<Value>(()).is_err());
    let value: Value = function.call((1,)).unwrap();
    assert!(crate::from_value::<DynamicTuple>(value).is_ok());

    // non-capturing closure == free function
    let function: Function = rune! {
        |a, b| a + b
    };

    assert!(function.call::<Value>((1,)).is_err());
    let value: Value = function.call((1, 2)).unwrap();
    assert_eq!(value.as_signed().unwrap(), 3);

    // closure with captures
    let function: Function = run(
        &context,
        "pub fn main(a, b) { || a + b }",
        (1, 2),
        false,
    )?;

    assert!(function.call::<Value>((1,)).is_err());
    let value: Value = function.call(()).unwrap();
    assert_eq!(value.as_signed().unwrap(), 3);
    Ok(())
}
