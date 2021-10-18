#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi::{CallContext, Env, JsNumber, JsObject, Result, Task, JsString};

struct AsyncTask(u32);

impl Task for AsyncTask {
  type Output = u32;
  type JsValue = JsNumber;

  fn compute(&mut self) -> Result<Self::Output> {
    use std::thread::sleep;
    use std::time::Duration;
    sleep(Duration::from_millis(self.0 as u64));
    Ok(self.0 * 2)
  }

  fn resolve(self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
    env.create_uint32(output)
  }
}

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
  exports.create_named_method("helloWorld", hello_world)?;

  Ok(())
}

#[js_function(0)]
fn hello_world(ctx: CallContext) -> Result<JsString> {
  let hello_world: JsString = ctx.env.create_string("Hello world")?;
  Ok(hello_world)
}


