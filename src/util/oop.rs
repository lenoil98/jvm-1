use crate::oop::{self, Oop, OopDesc};
use crate::runtime::{self, require_class3, JavaThread};
use crate::types::OopRef;
use crate::util;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref JAVA_LANG_STRING_VALUE_OFFSET: Mutex<Option<usize>> = { Mutex::new(None) };
}

pub fn set_java_lang_string_value_offset(offset: usize) {
    util::sync_call_ctx(&JAVA_LANG_STRING_VALUE_OFFSET, |v| {
        *v = Some(offset);
    });
}

pub fn is_str(v: OopRef) -> bool {
    let v = v.lock().unwrap();
    match &v.v {
        Oop::Inst(inst) => {
            let cls = inst.class.lock().unwrap();
            cls.name.as_slice() == b"java/lang/String"
        }
        _ => false,
    }
}

pub fn extract_str(v: OopRef) -> String {
    let offset: Option<usize> = util::sync_call(&JAVA_LANG_STRING_VALUE_OFFSET, |v| v.clone());
    let offset = offset.unwrap();

    let cls_string = require_class3(None, b"java/lang/String").unwrap();
    let value_ary = {
        let cls = cls_string.lock().unwrap();
        cls.get_field_value2(v.clone(), offset)
    };

    let value_ary = value_ary.lock().unwrap();
    match &value_ary.v {
        Oop::TypeArray(ary) => match ary {
            oop::TypeArrayValue::Char(ary) => String::from_utf16_lossy(ary.as_slice()),
            t => unreachable!("t = {:?}", t),
        },
        _ => unreachable!(),
    }

    /*
    if offset.is_some() {
        let offset = offset.unwrap();

        let cls_string = require_class3(None, b"java/lang/String").unwrap();
        let value_ary = {
            let cls = cls_string.lock().unwrap();
            cls.get_field_value2(v.clone(), offset)
        };

        let value_ary = value_ary.lock().unwrap();
        match &value_ary.v {
            Oop::TypeArray(ary) => match ary {
                oop::TypeArrayValue::Char(ary) => String::from_utf16_lossy(ary.as_slice()),
                t => unreachable!("t = {:?}", t),
            },
            _ => unreachable!(),
        }
    } else {
        let fid = {
            let v = v.lock().unwrap();
            match &v.v {
                Oop::Inst(inst) => {
                    let cls = inst.class.lock().unwrap();
                    cls.get_field_id(b"value", b"[C", false)
                }
                t => unreachable!("t = {:?}", t),
            }
        };

        let cls_string = require_class3(None, b"java/lang/String").unwrap();
        let value_ary = {
            let cls = cls_string.lock().unwrap();
            cls.get_field_value(v.clone(), fid)
        };

        let value_ary = value_ary.lock().unwrap();
        match &value_ary.v {
            Oop::TypeArray(ary) => match ary {
                oop::TypeArrayValue::Char(ary) => String::from_utf16_lossy(ary.as_slice()),
                t => unreachable!("t = {:?}", t),
            },
            _ => unreachable!(),
        }
    }
    */
}

pub fn if_acmpeq(v1: OopRef, v2: OopRef) -> bool {
    if Arc::ptr_eq(&v1, &v2) {
        true
    } else {
        if is_str(v2.clone()) && is_str(v1.clone()) {
            let v2 = extract_str(v2.clone());
            let v1 = extract_str(v1.clone());
            if v2 == v1 {
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

pub fn new_java_lang_string2(jt: &mut JavaThread, v: &str) -> OopRef {
    //build "char value[]"
    let chars: Vec<u16> = v.as_bytes().iter().map(|v| *v as u16).collect();
    let ary = OopDesc::char_ary_from1(chars.as_slice());

    //new String(char value[])
    let string_cls = require_class3(None, b"java/lang/String").unwrap();
    let string_oop = OopDesc::new_inst(string_cls.clone());
    let args = vec![string_oop.clone(), ary];
    runtime::java_call::invoke_ctor(jt, string_cls, b"([C)V", args);

    string_oop
}

pub fn new_java_lang_string3(jt: &mut JavaThread, v: &[u8]) -> OopRef {
    //build "char value[]"
    let v: Vec<u16> = v.iter().map(|v| *v as u16).collect();
    let ary = OopDesc::char_ary_from1(v.as_slice());

    //new String(char value[])
    let string_cls = require_class3(None, b"java/lang/String").unwrap();
    let string_oop = OopDesc::new_inst(string_cls.clone());
    let args = vec![string_oop.clone(), ary];
    runtime::java_call::invoke_ctor(jt, string_cls, b"([C)V", args);

    string_oop
}

pub fn hash_code(v: OopRef) -> u64 {
    {
        let v = v.lock().unwrap();
        match v.v {
            Oop::Null => return 0,
            Oop::Int(_) | Oop::Long(_) | Oop::Float(_) | Oop::Double(_) => unreachable!(),
            _ => (),
        }
    }

    if is_str(v.clone()) {
        let s = extract_str(v);
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        hasher.finish()
    } else {
        Arc::into_raw(v) as u64
    }
}
