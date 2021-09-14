use neon::{prelude::*, types::JsDate};

fn cp(mut cx: FunctionContext) -> JsResult<JsValue> {
    let v: Handle<JsValue> = cx.argument(0)?;
    if v.is_a::<JsNull, _>(&mut cx) {
        return Ok(v);
    }

    if v.is_a::<JsDate, _>(&mut cx) {
        let date: Handle<JsDate> = v.downcast_or_throw(&mut cx)?;
        let value = date.value(&mut cx);
        let new_date: Handle<JsDate> = cx.date(value).or_throw(&mut cx)?;
        return Ok(new_date.upcast());
    }

    if v.is_a::<JsArray, _>(&mut cx) {
        let arr: Handle<JsArray> = v.downcast_or_throw(&mut cx)?;
        let result = cx.empty_array();
        let mut i = 0;
        loop {
            // Since getting a property can trigger arbitrary code,
            // we have to re-check the length on every iteration.
            if i >= arr.len(&mut cx) {
                return Ok(result.downcast_or_throw(&mut cx)?);
            }
            let value = arr.get(&mut cx, i)?;
            result.set(&mut cx, i, value)?;
            i += 1;
        }
    }

    let obj = if v.is_a::<JsObject, _>(&mut cx) {
        cx.string("object")
    } else {
        cx.string("not an object")
    };
    return Ok(obj.upcast());
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("cp", cp)?;
    Ok(())
}
