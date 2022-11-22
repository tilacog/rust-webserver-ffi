mod webserver;
use neon::prelude::{Context, FunctionContext, JsNull, JsResult, ModuleContext, NeonResult};
use tokio::runtime::Runtime;

fn serve(mut cx: FunctionContext) -> JsResult<JsNull> {
    let rt = Runtime::new().expect("failed to start Tokio runtime");
    rt.block_on(webserver::serve());
    Ok(cx.null())
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("serve", serve)?;
    Ok(())
}
