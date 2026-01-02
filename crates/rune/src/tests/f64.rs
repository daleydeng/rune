prelude!();

#[test]
fn test_f64_ceil() -> Result<()> {
    let context = Context::with_default_modules()?;
    #[cfg(feature = "number-32")]
    {
        let value: f32 = run(&context, "(1.0 + std::f32::EPSILON).ceil()", (), true)?;
        assert_eq!(value, 2.0);
    }

    #[cfg(not(feature = "number-32"))]
    {
        let value: f64 = run(&context, "(1.0 + f64::EPSILON).ceil()", (), true)?;
        assert_eq!(value, 2.0);
    }

    Ok(())
}

#[test]
fn test_f64_consts() -> Result<()> {
    let context = Context::with_default_modules()?;

    #[cfg(feature = "number-32")]
    {
        let value: f32 = run(&context, "std::f32::consts::PI", (), true)?;
        assert_eq!(value, std::f32::consts::PI);
    }

    #[cfg(not(feature = "number-32"))]
    {
        let value: f64 = run(&context, "std::f64::consts::PI", (), true)?;
        assert_eq!(value, std::f64::consts::PI);
    }

    Ok(())
}

#[test]
fn test_f64_trig() -> Result<()> {
    let context = Context::with_default_modules()?;

    #[cfg(feature = "number-32")]
    {
        let value: f32 = run(
            &context,
            r#"
                let x = std::f32::consts::FRAC_PI_4;
                (x.tan() - 1.0).abs()
            "#,
            (),
            true,
        )?;
        assert!(value < 1e-6);
    }

    #[cfg(not(feature = "number-32"))]
    {
        let value: f64 = run(
            &context,
            r#"
                let x = std::f64::consts::FRAC_PI_4;
                (x.tan() - 1.0).abs()
            "#,
            (),
            true,
        )?;
        assert!(value < 1e-14);
    }

    Ok(())
}
