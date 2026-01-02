prelude!();

use core::ops::ControlFlow;

use crate::runtime::SignedType;

#[test]
fn custom_try() -> Result<()> {
    #[derive(Any)]
    struct CustomResult(bool);

    let mut module = Module::new();

    module.ty::<CustomResult>()?;

    module.associated_function(&Protocol::TRY, |r: CustomResult| {
        if r.0 {
            ControlFlow::Continue(SignedType::from(42))
        } else {
            ControlFlow::Break(Err::<Value, _>(SignedType::from(0)))
        }
    })?;

    let n: u32 = rune_n! {
        mod module,
        (CustomResult(true),),
        pub fn main(r) { r? }
    };

    assert_eq!(n, 42);

    let result: Result<(), SignedType> = rune_n! {
        mod module,
        (CustomResult(false),),
        pub fn main(r) { r? }
    };

    assert_eq!(result, Err(0));
    Ok(())
}
