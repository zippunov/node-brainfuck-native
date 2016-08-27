#[macro_use]
extern crate neon;
extern crate bfck;

use neon::vm::{Call, JsResult};
use neon::js::JsString;
use neon::js::error::{JsError, Kind};
use neon::mem::Handle;
use bfck::bfck::BFBox;
use std::io::Cursor;

fn run_bfck(call: Call) -> JsResult<JsString> {
    let scope = call.scope;
    let code_handle: Handle<JsString> = try!(try!(call.arguments.require(scope, 0)).check::<JsString>());
    let code = String::from(code_handle.value());
    let input_handle: Handle<JsString> = try!(try!(call.arguments.require(scope, 1)).check::<JsString>());
    let input = String::from(input_handle.value());
    let mut read_buff = Cursor::new(input);
    let mut write_buff = Cursor::new(Vec::new());

    let mut bfck_box = match BFBox::new(&code) {
        Ok(bfck_box) => bfck_box,
        _ => return JsError::throw(Kind::TypeError, "Failed to parse code"),
    };

    match bfck_box.run(&mut read_buff, &mut write_buff) {
        Ok(()) => true,
        _ => return JsError::throw(Kind::TypeError, "Execution exception"),
    };

    let result_string = String::from_utf8(write_buff.into_inner()).unwrap();
    Ok(JsString::new(scope, &result_string).unwrap())
}

register_module!(m, {
    m.export("runCode", run_bfck)
});
