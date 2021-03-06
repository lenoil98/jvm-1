#![allow(non_snake_case)]
use crate::native::{new_fn, JNIEnv, JNINativeMethod, JNIResult};
use crate::oop::{self, Oop};
use crate::util;
use classfile::constant_pool;

pub fn get_native_methods() -> Vec<JNINativeMethod> {
    vec![new_fn(
        "getUTF8At0",
        "(Ljava/lang/Object;I)Ljava/lang/String;",
        Box::new(jvm_getUTF8At0),
    )]
}

fn jvm_getUTF8At0(_env: JNIEnv, args: &[Oop]) -> JNIResult {
    let _this = args.get(0).unwrap();
    let cp_oop = args.get(1).unwrap();
    let index = {
        let index = args.get(2).unwrap();
        index.extract_int()
    };

    let rf = cp_oop.extract_ref();
    let mirror = rf.extract_mirror();
    let target = mirror.target.clone().unwrap();
    let cls = target.get_class();
    match &cls.kind {
        oop::class::ClassKind::Instance(inst) => {
            let cp = &inst.class_file.cp;
            let s = constant_pool::get_utf8(cp, index as usize);
            let s = unsafe { std::str::from_utf8_unchecked(s.as_slice()) };
            let r = util::oop::new_java_lang_string2(s);
            Ok(Some(r))
        }
        _ => unimplemented!(),
    }
}
