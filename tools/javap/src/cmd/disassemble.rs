use crate::cmd::Cmd;
use crate::misc::SysInfo;
use crate::sd::{ClassInfoSerde, FieldInfoSerde, LineNumberSerde, MethodInfoSerde, SysInfoSerde};
use crate::template;
use crate::trans::{self, AccessFlagHelper};
use clap::ArgMatches;
use classfile::flags as access_flags;
use classfile::ClassFile;

pub struct Disassemble {
    show_access_flags: u16,

    enable_verbose: bool,
    enable_line_number: bool,
    enable_code: bool,
    enable_sys_info: bool,
    enable_inner_signature: bool,
}

impl Disassemble {
    pub fn new(m: &ArgMatches) -> Option<Self> {
        let acc_flags = Self::build_show_access_flags(m);

        let enable_verbose = m.is_present("verbose");
        let enable_line_number = enable_verbose || m.is_present("line_number");
        let enable_code = enable_verbose || m.is_present("disassemble");
        let enable_sys_info = enable_verbose || m.is_present("sysinfo");
        let enable_inner_signature = enable_verbose || m.is_present("signatures");

        Some(Self {
            show_access_flags: acc_flags,

            enable_verbose,
            enable_line_number,
            enable_code,
            enable_sys_info,
            enable_inner_signature,
        })
    }
}

impl Cmd for Disassemble {
    fn run(&self, si: &SysInfo, cf: ClassFile) {
        self.do_render(si, cf);
    }
}

impl Disassemble {
    fn do_render(&self, si: &SysInfo, cf: ClassFile) {
        let reg = template::get_engine();

        let sys_info = self.build_sys_info(si, &cf);
        let source_file = trans::class_source_file(&cf);
        let class_head = self.build_class_define(&cf);
        let fields = self.class_fields(&cf);
        let methods = self.build_methods(&cf);

        let data = ClassInfoSerde {
            enable_sys_info: self.enable_sys_info,
            sys_info,
            source_file,
            class_head,
            fields,
            methods,
        };

        println!("{}", reg.render_template(template::CLASS, &data).unwrap());
    }

    fn class_fields(&self, cf: &ClassFile) -> Vec<FieldInfoSerde> {
        let fields = trans::class_fields(&cf, self.show_access_flags);
        fields
            .iter()
            .map(|it| FieldInfoSerde {
                desc: it.desc.clone(),
                signature: it.signature.clone(),
                enable_inner_signature: self.enable_inner_signature,
            })
            .collect()
    }
}

impl Disassemble {
    fn build_show_access_flags(m: &ArgMatches) -> u16 {
        let mut flags = 0;

        if m.is_present("public") {
            flags = access_flags::ACC_PUBLIC;
        }

        if m.is_present("protected") {
            flags = access_flags::ACC_PROTECTED;
        }

        if m.is_present("private") {
            flags = access_flags::ACC_PRIVATE;
        }

        flags
    }

    fn build_class_define(&self, cf: &ClassFile) -> String {
        let mut head_parts = vec![];

        let class_flags = trans::class_access_flags(&cf);
        let this_class = trans::class_this_class(&cf);
        head_parts.push(class_flags);
        head_parts.push(this_class.clone());

        if cf.acc_flags.is_interface() {
            if cf.interfaces.len() != 0 {
                head_parts.push("extends".to_string());

                let parent_interfaces = trans::class_parent_interfaces(&cf).join(", ");
                head_parts.push(parent_interfaces);
            }
        } else if cf.acc_flags.is_enum() {
            head_parts.push("extends".to_string());

            let super_class = {
                let mut super_class = trans::class_super_class(&cf);
                super_class.push_str("<");
                super_class.push_str(this_class.as_str());
                super_class.push_str(">");

                super_class
            };

            head_parts.push(super_class);
        } else {
            let super_class = trans::class_super_class(&cf);
            if super_class != "java.lang.Object" {
                head_parts.push("extends".to_string());
                head_parts.push(super_class);
            }

            if cf.interfaces.len() != 0 {
                head_parts.push("implements".to_string());

                let parent_interfaces = trans::class_parent_interfaces(&cf).join(", ");
                head_parts.push(parent_interfaces);
            }
        }

        head_parts.join(" ")
    }

    fn build_sys_info(&self, si: &SysInfo, cf: &ClassFile) -> SysInfoSerde {
        if self.enable_sys_info {
            let source_file = trans::class_source_file(&cf);
            SysInfoSerde {
                class_file: si.class_file.clone(),
                last_modified: si.last_modified.clone(),
                size: si.size,
                checksum: si.checksum.clone(),
                compiled_from: source_file,
            }
        } else {
            SysInfoSerde::default()
        }
    }

    fn build_methods(&self, cf: &ClassFile) -> Vec<MethodInfoSerde> {
        let is_interface = cf.acc_flags.is_interface();

        let methods = trans::class_methods(
            cf,
            self.enable_line_number,
            self.enable_code,
            self.show_access_flags,
        );

        methods
            .iter()
            .map(|it| {
                if is_interface {
                    MethodInfoSerde {
                        desc: it.desc.clone(),
                        line_number_table: vec![],
                        code: Default::default(),
                        signature: it.signature.clone(),
                        enable_line_number: false,
                        enable_code: false,
                        enable_inner_signature: self.enable_inner_signature,
                    }
                } else {
                    let enable_line_number = self.enable_line_number;
                    let enable_code = self.enable_code;

                    let line_number_table: Vec<LineNumberSerde> = if enable_line_number {
                        it.line_num_table
                            .iter()
                            .map(|it| LineNumberSerde {
                                start_pc: it.start_pc,
                                line_number: it.number,
                            })
                            .collect()
                    } else {
                        vec![]
                    };

                    let code = if enable_code {
                        let mut code = it.code.clone();
                        code.enable_verbose = self.enable_verbose;
                        code
                    } else {
                        Default::default()
                    };

                    MethodInfoSerde {
                        desc: it.desc.clone(),
                        line_number_table,
                        code,
                        signature: it.signature.clone(),
                        enable_line_number,
                        enable_code,
                        enable_inner_signature: self.enable_inner_signature,
                    }
                }
            })
            .collect()
    }
}
