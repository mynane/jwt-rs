mod jwt;

use neon::prelude::*;
use crate::jwt::jwt::JWT;

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    // cx.export_function("encode", encode)?;
    JWT::new(cx);

    Ok(())
}
