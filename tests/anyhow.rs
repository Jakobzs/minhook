use anyhow::Result;
use minhook::MinHook;

#[test]
fn test_anyhow() -> Result<()> {
    unsafe {
        MinHook::enable_all_hooks()?;
    }

    Ok(())
}
