use std::env;

pub struct JavaCommand {
    pub jar_path: Option<String>,
    pub main_class: Option<String>,
    pub vm_options: Vec<String>,
    pub app_args: Vec<String>,
}

impl JavaCommand {
    pub fn parse() -> Self {
        let args: Vec<String> = env::args().skip(1).collect();
        let mut jar_path = None;
        let mut main_class = None;
        let mut vm_options = Vec::new();
        let mut app_args = Vec::new();
        let mut found_main = false;
        let mut next_is_jar = false;

        for arg in args {
            if next_is_jar {
                jar_path = Some(arg);
                main_class = None; // jar模式下没有主类名
                found_main = true;
                next_is_jar = false;
                continue;
            }

            if !found_main {
                match arg.as_str() {
                    "-jar" => {
                        next_is_jar = true;
                        vm_options.push(arg);
                    }
                    s if s.ends_with(".jar") => {
                        // 直接传递的jar文件
                        jar_path = Some(arg.clone());
                        main_class = None;
                        found_main = true;
                    }
                    s if !s.starts_with("-") => {
                        // 普通类名
                        main_class = Some(arg.clone());
                        found_main = true;
                    }
                    _ => {
                        vm_options.push(arg);
                    }
                }
            } else {
                app_args.push(arg);
            }
        }

        Self {
            jar_path,
            main_class,
            vm_options,
            app_args,
        }
    }

    pub fn get_entry_point(&self) -> Option<String> {
        if let Some(jar) = &self.jar_path {
            Some(format!("jar:{}", jar))
        } else {
            self.main_class.clone()
        }
    }

    pub fn is_jar_mode(&self) -> bool {
        self.jar_path.is_some()
    }
}
