/// JVM 可抛出异常/错误的统一枚举
/// 
/// 这是 Java 异常处理机制的 Rust 实现，严格按照 JVM 规范分类：
/// - Exception: 程序可处理的异常（分为运行时异常和检查异常）
/// - Error: 严重系统错误，应用程序不应捕获
#[derive(Debug, Clone, PartialEq)]
pub enum Throwable {
    /// 程序异常（可恢复）
    Exception(Exception),
    /// 系统错误（不可恢复）
    Error(JvmError),
}

/// 程序异常类型
///
/// 对应 Java 的 Exception 类及其子类，严格按照 JVM 规范分为：
/// - RuntimeException: 运行时异常，无需在方法签名中声明
/// - CheckedException: 检查异常，必须在方法签名中声明
#[derive(Debug, Clone, PartialEq)]
pub enum Exception {
    // ========== RuntimeException（运行时异常）==========
    // 这些异常继承自 java.lang.RuntimeException
    
    /// 空指针异常
    /// 当应用程序试图在需要对象的地方使用 null 时抛出
    NullPointerException(String),
    
    /// 数组索引越界异常
    /// 使用非法索引访问数组时抛出（索引为负或大于等于数组大小）
    ArrayIndexOutOfBoundsException {
        index: i32,
        length: i32,
        message: String,
    },
    
    /// 类转换异常
    /// 试图将对象强制转换为不是实例的子类时抛出
    ClassCastException {
        from_type: String,
        to_type: String,
        message: String,
    },
    
    /// 算术异常
    /// 出现异常的算术条件时抛出（如整数除以零）
    ArithmeticException(String),
    
    /// 非法参数异常
    /// 向方法传递非法或不合适的参数时抛出
    IllegalArgumentException(String),
    
    /// 非法状态异常
    /// 在非法或不适当的时间调用方法时抛出
    IllegalStateException(String),
    
    /// 索引越界异常
    /// 指示某种索引（如数组、字符串或向量）越界时抛出
    IndexOutOfBoundsException(String),
    
    /// 负数组大小异常
    /// 当应用程序试图创建大小为负的数组时抛出
    NegativeArraySizeException(String),
    
    /// 数字格式异常
    /// 当应用程序试图将字符串转换为数值类型，但字符串格式不匹配时抛出
    NumberFormatException(String),
    
    /// 并发修改异常
    /// 当检测到对象不允许并发修改时抛出
    ConcurrentModificationException(String),
    
    /// 不支持的操作异常
    /// 当不支持请求的操作时抛出
    UnsupportedOperationException(String),
    
    // ========== CheckedException（检查异常）==========
    // 这些异常继承自 java.lang.Exception 但不是 RuntimeException
    
    /// IO异常
    /// 发生某种 I/O 异常时抛出的信号
    IOException {
        kind: IOErrorKind,
        message: String,
        path: Option<String>,
    },
    
    /// 类未找到异常
    /// 当应用程序试图通过字符串名加载类但找不到类定义时抛出
    ClassNotFoundException {
        class_name: String,
        cause: Option<String>,
        message: String,
    },
    
    /// 文件未找到异常
    /// 试图打开指定路径名表示的文件失败时抛出
    FileNotFoundException {
        path: String,
        message: String,
    },
    
    /// 中断异常
    /// 当线程在等待、睡眠或其他方式占用时被中断时抛出
    InterruptedException(String),
    
    /// 安全异常
    /// 由安全管理器抛出的异常，指示违反安全规定
    SecurityException(String),
    
    /// 超时异常
    /// 阻塞操作超时时抛出
    TimeoutException(String),
    
    /// 解析异常
    /// 解析字符串或数据时发生错误
    ParseException {
        target: String,
        position: usize,
        message: String,
    },
    
    /// SQL 异常
    /// 提供关于数据库访问错误或其他错误的信息
    SQLException {
        sql_state: Option<String>,
        error_code: Option<i32>,
        message: String,
    },
    
    /// 反射操作异常
    /// 反射操作的基础异常
    ReflectiveOperationException(String),
}

/// 虚拟机错误类型
///
/// 对应 Java 的 Error 类及其子类，表示严重问题：
/// - 应用程序不应捕获这些错误
/// - 通常表示系统资源耗尽或 JVM 内部错误
#[derive(Debug, Clone, PartialEq)]
pub enum JvmError {
    // ========== VirtualMachineError（虚拟机错误）==========
    // 这些错误继承自 java.lang.VirtualMachineError
    
    /// 内存溢出错误
    /// 当 JVM 无法分配对象所需内存，且垃圾收集器无法提供更多内存时抛出
    OutOfMemoryError {
        heap_size: u64,
        requested: u64,
        message: String,
    },
    
    /// 栈溢出错误
    /// 当应用程序递归太深而发生栈溢出时抛出
    StackOverflowError {
        thread_name: String,
        stack_depth: usize,
        message: String,
    },
    
    /// 内部错误
    /// 发生 JVM 内部错误或资源限制时抛出
    InternalError {
        message: String,
        line_number: u32,
        file_name: String,
    },
    
    /// 未知错误
    /// 发生未知但严重的异常时抛出
    UnknownError(String),
    
    // ========== LinkageError（链接错误）==========
    // 这些错误继承自 java.lang.LinkageError
    
    /// 类定义未找到错误
    /// 当 JVM 或 ClassLoader 实例试图加载类定义但未找到时抛出
    NoClassDefFoundError {
        class_name: String,
        cause: Option<String>,
        message: String,
    },
    
    /// 类格式错误
    /// 当 JVM 试图读取类文件并确定文件格式不正确时抛出
    ClassFormatError {
        class_name: String,
        message: String,
    },
    
    /// 不支持的类版本错误
    /// 当 JVM 试图读取类文件，但发现文件的主次版本号不受支持时抛出
    UnsupportedClassVersionError {
        class_name: String,
        version: String,
        supported_versions: Option<String>,
        message: String,
    },
    
    /// 字段未找到错误
    /// 当应用程序试图访问类的指定字段，但该类不再包含该字段时抛出
    NoSuchFieldError {
        class_name: String,
        field_name: String,
        field_descriptor: Option<String>,
        message: String,
    },
    
    /// 方法未找到错误
    /// 当应用程序试图调用类的指定方法，但该类不再包含该方法时抛出
    NoSuchMethodError {
        class_name: String,
        method_name: String,
        method_descriptor: Option<String>,
        message: String,
    },
    
    /// 抽象方法错误
    /// 当应用程序试图调用抽象方法时抛出
    AbstractMethodError {
        class_name: String,
        method_name: String,
        message: String,
    },
    
    /// 非法访问错误
    /// 当应用程序试图访问或修改它无法访问的字段，或调用它无法访问的方法时抛出
    IllegalAccessError {
        class_name: String,
        member_name: String,
        member_type: String,
        message: String,
    },
    
    /// 实例化错误
    /// 当应用程序试图使用 Class 的 newInstance 方法实例化一个抽象类或接口时抛出
    InstantiationError {
        class_name: String,
        reason: String,
        message: String,
    },
    
    /// 不兼容的类更改错误
    /// 当某个类的定义发生不兼容更改时抛出
    IncompatibleClassChangeError {
        class_name: String,
        change_type: ClassChangeType,
        message: String,
    },
    
    /// 不满足链接错误
    /// 当链接器无法解析符号引用时抛出，通常涉及本地方法
    UnsatisfiedLinkError {
        library_name: String,
        function_name: Option<String>,
        os_error: Option<String>,
        message: String,
    },
    
    /// 验证错误
    /// 当类文件验证失败时抛出
    VerifyError {
        class_name: String,
        bytecode_offset: u32,
        verification_error: String,
        message: String,
    },
    
    /// 引导方法错误
    /// 当 invokedynamic 指令的引导方法出错时抛出
    BootstrapMethodError {
        method_name: String,
        cause: Option<String>,
        message: String,
    },
    
    // ========== Thread Error（线程错误）==========
    
    /// 线程死亡错误
    /// 当调用 Thread.stop 方法时抛出（已弃用）
    ThreadDeath(String),
    
    // ========== Assertion Error（断言错误）==========
    
    /// 断言错误
    /// 当断言语句失败时抛出
    AssertionError {
        condition: String,
        file: Option<String>,
        line: Option<u32>,
        message: String,
    },
    
    // ========== Other Critical Errors（其他严重错误）==========
    
    /// IO 错误
    /// 发生严重 I/O 错误时抛出
    IOError {
        kind: IOErrorKind,
        message: String,
        path: Option<String>,
    },
    
    /// 异常初始化器错误
    /// 当静态初始化器中发生意外异常时抛出
    ExceptionInInitializerError {
        class_name: String,
        cause: String,
        message: String,
    },
}

/// 类不兼容更改类型
#[derive(Debug, Clone, PartialEq)]
pub enum ClassChangeType {
    /// 添加/删除方法
    MethodChanged,
    /// 添加/删除字段
    FieldChanged,
    /// 更改类层次结构（如父类变更）
    HierarchyChanged,
    /// 更改访问权限
    AccessChanged,
    /// 更改方法签名
    SignatureChanged,
    /// 更改类/接口状态
    ClassToInterface,
    InterfaceToClass,
    /// 添加/删除接口实现
    InterfaceChanged,
    /// 静态成员变为实例成员，或反之
    StaticNonStaticChanged,
}

/// IO错误类型分类
#[derive(Debug, Clone, PartialEq)]
pub enum IOErrorKind {
    NotFound,
    PermissionDenied,
    ConnectionRefused,
    ConnectionReset,
    ConnectionAborted,
    NotConnected,
    AddrInUse,
    AddrNotAvailable,
    BrokenPipe,
    AlreadyExists,
    WouldBlock,
    InvalidInput,
    InvalidData,
    TimedOut,
    WriteZero,
    Interrupted,
    UnexpectedEof,
    Unsupported,
    Other,
}

impl std::fmt::Display for IOErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IOErrorKind::NotFound => write!(f, "File not found"),
            IOErrorKind::PermissionDenied => write!(f, "Permission denied"),
            IOErrorKind::ConnectionRefused => write!(f, "Connection refused"),
            IOErrorKind::ConnectionReset => write!(f, "Connection reset"),
            IOErrorKind::ConnectionAborted => write!(f, "Connection aborted"),
            IOErrorKind::NotConnected => write!(f, "Not connected"),
            IOErrorKind::AddrInUse => write!(f, "Address already in use"),
            IOErrorKind::AddrNotAvailable => write!(f, "Address not available"),
            IOErrorKind::BrokenPipe => write!(f, "Broken pipe"),
            IOErrorKind::AlreadyExists => write!(f, "File already exists"),
            IOErrorKind::WouldBlock => write!(f, "Operation would block"),
            IOErrorKind::InvalidInput => write!(f, "Invalid input"),
            IOErrorKind::InvalidData => write!(f, "Invalid data"),
            IOErrorKind::TimedOut => write!(f, "Operation timed out"),
            IOErrorKind::WriteZero => write!(f, "Write zero bytes"),
            IOErrorKind::Interrupted => write!(f, "Operation interrupted"),
            IOErrorKind::UnexpectedEof => write!(f, "Unexpected end of file"),
            IOErrorKind::Unsupported => write!(f, "Unsupported operation"),
            IOErrorKind::Other => write!(f, "Other I/O error"),
        }
    }
}

// ========== 实现部分 ==========

impl Throwable {
    /// 获取错误信息
    pub fn message(&self) -> String {
        match self {
            Throwable::Exception(e) => e.message(),
            Throwable::Error(e) => e.message(),
        }
    }
    
    /// 判断是否为运行时异常
    pub fn is_runtime_exception(&self) -> bool {
        match self {
            Throwable::Exception(e) => e.is_runtime_exception(),
            _ => false,
        }
    }
    
    /// 判断是否为检查异常
    pub fn is_checked_exception(&self) -> bool {
        match self {
            Throwable::Exception(e) => e.is_checked_exception(),
            _ => false,
        }
    }
    
    /// 判断是否为错误（Error）
    pub fn is_error(&self) -> bool {
        matches!(self, Throwable::Error(_))
    }
    
    /// 判断是否为虚拟机错误
    pub fn is_virtual_machine_error(&self) -> bool {
        match self {
            Throwable::Error(e) => e.is_virtual_machine_error(),
            _ => false,
        }
    }
    
    /// 判断是否为链接错误
    pub fn is_linkage_error(&self) -> bool {
        match self {
            Throwable::Error(e) => e.is_linkage_error(),
            _ => false,
        }
    }
    
    /// 获取异常/错误的简短名称
    pub fn type_name(&self) -> &'static str {
        match self {
            Throwable::Exception(e) => e.type_name(),
            Throwable::Error(e) => e.type_name(),
        }
    }
}

impl Exception {
    /// 获取异常信息
    pub fn message(&self) -> String {
        match self {
            Exception::NullPointerException(msg)
            | Exception::ArithmeticException(msg)
            | Exception::IllegalArgumentException(msg)
            | Exception::IllegalStateException(msg)
            | Exception::IndexOutOfBoundsException(msg)
            | Exception::NegativeArraySizeException(msg)
            | Exception::NumberFormatException(msg)
            | Exception::ConcurrentModificationException(msg)
            | Exception::UnsupportedOperationException(msg)
            | Exception::InterruptedException(msg)
            | Exception::SecurityException(msg)
            | Exception::TimeoutException(msg)
            | Exception::ReflectiveOperationException(msg) => msg.clone(),
            
            Exception::ArrayIndexOutOfBoundsException { index, length, message } => {
                format!("{} (index={}, length={})", message, index, length)
            }
            
            Exception::ClassCastException { from_type, to_type, message } => {
                format!("{} (from={}, to={})", message, from_type, to_type)
            }
            
            Exception::IOException { kind, message, path } => {
                if let Some(p) = path {
                    format!("{}: {} at '{}'", kind, message, p)
                } else {
                    format!("{}: {}", kind, message)
                }
            }
            
            Exception::ClassNotFoundException { class_name, cause, message } => {
                if let Some(c) = cause {
                    format!("{} (class={}, cause={})", message, class_name, c)
                } else {
                    format!("{} (class={})", message, class_name)
                }
            }
            
            Exception::FileNotFoundException { path, message } => {
                format!("{}: '{}'", message, path)
            }
            
            Exception::ParseException { target, position, message } => {
                format!("{} at position {} in '{}'", message, position, target)
            }
            
            Exception::SQLException { sql_state, error_code, message } => {
                let mut details = Vec::new();
                if let Some(code) = error_code {
                    details.push(format!("error code: {}", code));
                }
                if let Some(state) = sql_state {
                    details.push(format!("SQL state: {}", state));
                }
                if details.is_empty() {
                    message.clone()
                } else {
                    format!("{} ({})", message, details.join(", "))
                }
            }
        }
    }
    
    /// 获取异常类型名称
    pub fn type_name(&self) -> &'static str {
        match self {
            Exception::NullPointerException(_) => "NullPointerException",
            Exception::ArrayIndexOutOfBoundsException { .. } => "ArrayIndexOutOfBoundsException",
            Exception::ClassCastException { .. } => "ClassCastException",
            Exception::ArithmeticException(_) => "ArithmeticException",
            Exception::IllegalArgumentException(_) => "IllegalArgumentException",
            Exception::IllegalStateException(_) => "IllegalStateException",
            Exception::IndexOutOfBoundsException(_) => "IndexOutOfBoundsException",
            Exception::NegativeArraySizeException(_) => "NegativeArraySizeException",
            Exception::NumberFormatException(_) => "NumberFormatException",
            Exception::ConcurrentModificationException(_) => "ConcurrentModificationException",
            Exception::UnsupportedOperationException(_) => "UnsupportedOperationException",
            Exception::IOException { .. } => "IOException",
            Exception::ClassNotFoundException { .. } => "ClassNotFoundException",
            Exception::FileNotFoundException { .. } => "FileNotFoundException",
            Exception::InterruptedException(_) => "InterruptedException",
            Exception::SecurityException(_) => "SecurityException",
            Exception::TimeoutException(_) => "TimeoutException",
            Exception::ParseException { .. } => "ParseException",
            Exception::SQLException { .. } => "SQLException",
            Exception::ReflectiveOperationException(_) => "ReflectiveOperationException",
        }
    }
    
    /// 判断是否为运行时异常
    pub fn is_runtime_exception(&self) -> bool {
        matches!(
            self,
            Exception::NullPointerException(_)
                | Exception::ArrayIndexOutOfBoundsException { .. }
                | Exception::ClassCastException { .. }
                | Exception::ArithmeticException(_)
                | Exception::IllegalArgumentException(_)
                | Exception::IllegalStateException(_)
                | Exception::IndexOutOfBoundsException(_)
                | Exception::NegativeArraySizeException(_)
                | Exception::NumberFormatException(_)
                | Exception::ConcurrentModificationException(_)
                | Exception::UnsupportedOperationException(_)
        )
    }
    
    /// 判断是否为检查异常
    pub fn is_checked_exception(&self) -> bool {
        !self.is_runtime_exception()
    }
}

impl JvmError {
    /// 获取错误信息
    pub fn message(&self) -> String {
        match self {
            JvmError::OutOfMemoryError { heap_size, requested, message } => {
                format!("{} (heap={}, requested={})", message, heap_size, requested)
            }
            
            JvmError::StackOverflowError { thread_name, stack_depth, message } => {
                format!("{} (thread={}, depth={})", message, thread_name, stack_depth)
            }
            
            JvmError::InternalError { message, line_number, file_name } => {
                format!("{} (at {}:{})", message, file_name, line_number)
            }
            
            JvmError::UnknownError(msg)
            | JvmError::ThreadDeath(msg) => msg.clone(),
            
            JvmError::NoClassDefFoundError { class_name, cause, message } => {
                if let Some(c) = cause {
                    format!("{} (class={}, cause={})", message, class_name, c)
                } else {
                    format!("{} (class={})", message, class_name)
                }
            }
            
            JvmError::ClassFormatError { class_name, message } => {
                 format!("{} (class={})", message, class_name)
            }
            
            JvmError::UnsupportedClassVersionError { class_name, version, supported_versions, message } => {
                if let Some(supported) = supported_versions {
                    format!("{} (class={}, version={}, supported={})", message, class_name, version, supported)
                } else {
                    format!("{} (class={}, version={})", message, class_name, version)
                }
            }
            
            JvmError::NoSuchFieldError { class_name, field_name, field_descriptor, message } => {
                if let Some(desc) = field_descriptor {
                    format!("{} (class={}, field={}, descriptor={})", message, class_name, field_name, desc)
                } else {
                    format!("{} (class={}, field={})", message, class_name, field_name)
                }
            }
            
            JvmError::NoSuchMethodError { class_name, method_name, method_descriptor, message } => {
                if let Some(desc) = method_descriptor {
                    format!("{} (class={}, method={}, descriptor={})", message, class_name, method_name, desc)
                } else {
                    format!("{} (class={}, method={})", message, class_name, method_name)
                }
            }
            
            JvmError::AbstractMethodError { class_name, method_name, message } => {
                format!("{} (class={}, method={})", message, class_name, method_name)
            }
            
            JvmError::IllegalAccessError { class_name, member_name, member_type, message } => {
                format!("{} (class={}, {}={})", message, class_name, member_type, member_name)
            }
            
            JvmError::InstantiationError { class_name, reason, message } => {
                format!("{} (class={}, reason={})", message, class_name, reason)
            }
            
            JvmError::IncompatibleClassChangeError { class_name, change_type, message } => {
                format!("{} (class={}, change={:?})", message, class_name, change_type)
            }
            
            JvmError::UnsatisfiedLinkError { library_name, function_name, os_error, message } => {
                let mut details = vec![format!("library={}", library_name)];
                if let Some(func) = function_name {
                    details.push(format!("function={}", func));
                }
                if let Some(error) = os_error {
                    details.push(format!("os_error={}", error));
                }
                format!("{} ({})", message, details.join(", "))
            }
            
            JvmError::VerifyError { class_name, bytecode_offset, verification_error, message } => {
                format!("{} (class={}, offset={}, error={})", message, class_name, bytecode_offset, verification_error)
            }
            
            JvmError::BootstrapMethodError { method_name, cause, message } => {
                if let Some(c) = cause {
                    format!("{} (method={}, cause={})", message, method_name, c)
                } else {
                    format!("{} (method={})", message, method_name)
                }
            }
            
            JvmError::AssertionError { condition, file, line, message } => {
                let location = match (file, line) {
                    (Some(f), Some(l)) => format!(" at {}:{}", f, l),
                    (Some(f), None) => format!(" at {}", f),
                    (None, Some(l)) => format!(" at line {}", l),
                    (None, None) => String::new(),
                };
                format!("{} (condition={}){}", message, condition, location)
            }
            
            JvmError::IOError { kind, message, path } => {
                if let Some(p) = path {
                    format!("{}: {} at '{}'", kind, message, p)
                } else {
                    format!("{}: {}", kind, message)
                }
            }
            
            JvmError::ExceptionInInitializerError { class_name, cause, message } => {
                format!("{} (class={}, cause={})", message, class_name, cause)
            }
        }
    }
    
    /// 获取错误类型名称
    pub fn type_name(&self) -> &'static str {
        match self {
            JvmError::OutOfMemoryError { .. } => "OutOfMemoryError",
            JvmError::StackOverflowError { .. } => "StackOverflowError",
            JvmError::InternalError { .. } => "InternalError",
            JvmError::UnknownError(_) => "UnknownError",
            JvmError::NoClassDefFoundError { .. } => "NoClassDefFoundError",
            JvmError::ClassFormatError { .. } => "ClassFormatError",
            JvmError::UnsupportedClassVersionError { .. } => "UnsupportedClassVersionError",
            JvmError::NoSuchFieldError { .. } => "NoSuchFieldError",
            JvmError::NoSuchMethodError { .. } => "NoSuchMethodError",
            JvmError::AbstractMethodError { .. } => "AbstractMethodError",
            JvmError::IllegalAccessError { .. } => "IllegalAccessError",
            JvmError::InstantiationError { .. } => "InstantiationError",
            JvmError::IncompatibleClassChangeError { .. } => "IncompatibleClassChangeError",
            JvmError::UnsatisfiedLinkError { .. } => "UnsatisfiedLinkError",
            JvmError::VerifyError { .. } => "VerifyError",
            JvmError::BootstrapMethodError { .. } => "BootstrapMethodError",
            JvmError::ThreadDeath(_) => "ThreadDeath",
            JvmError::AssertionError { .. } => "AssertionError",
            JvmError::IOError { .. } => "IOError",
            JvmError::ExceptionInInitializerError { .. } => "ExceptionInInitializerError",
        }
    }
    
    /// 判断是否为链接错误
    pub fn is_linkage_error(&self) -> bool {
        matches!(
            self,
            JvmError::NoClassDefFoundError { .. }
                | JvmError::ClassFormatError { .. }
                | JvmError::UnsupportedClassVersionError { .. }
                | JvmError::NoSuchFieldError { .. }
                | JvmError::NoSuchMethodError { .. }
                | JvmError::AbstractMethodError { .. }
                | JvmError::IllegalAccessError { .. }
                | JvmError::InstantiationError { .. }
                | JvmError::IncompatibleClassChangeError { .. }
                | JvmError::UnsatisfiedLinkError { .. }
                | JvmError::VerifyError { .. }
                | JvmError::BootstrapMethodError { .. }
        )
    }
    
    /// 判断是否为虚拟机错误
    pub fn is_virtual_machine_error(&self) -> bool {
        matches!(
            self,
            JvmError::OutOfMemoryError { .. }
                | JvmError::StackOverflowError { .. }
                | JvmError::InternalError { .. }
                | JvmError::UnknownError(_)
        )
    }
}

// ========== Display 实现 ==========

impl std::fmt::Display for Throwable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Throwable::Exception(e) => write!(f, "Exception in thread: {}", e.message()),
            Throwable::Error(e) => write!(f, "Error: {}", e.message()),
        }
    }
}

impl std::fmt::Display for Exception {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.type_name(), self.message())
    }
}

impl std::fmt::Display for JvmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.type_name(), self.message())
    }
}

impl std::error::Error for Throwable {}
impl std::error::Error for Exception {}
impl std::error::Error for JvmError {}

// ========== From 实现（便于转换）===========

impl From<Exception> for Throwable {
    fn from(e: Exception) -> Self {
        Throwable::Exception(e)
    }
}

impl From<JvmError> for Throwable {
    fn from(e: JvmError) -> Self {
        Throwable::Error(e)
    }
}

// ========== 便捷构造函数 ==========

impl Exception {
    /// 创建空指针异常
    pub fn null_pointer(msg: impl Into<String>) -> Self {
        Exception::NullPointerException(msg.into())
    }
    
    /// 创建数组索引越界异常
    pub fn array_index_out_of_bounds(index: i32, length: i32, msg: impl Into<String>) -> Self {
        Exception::ArrayIndexOutOfBoundsException {
            index,
            length,
            message: msg.into(),
        }
    }
    
    /// 创建类转换异常
    pub fn class_cast(from: impl Into<String>, to: impl Into<String>, msg: impl Into<String>) -> Self {
        Exception::ClassCastException {
            from_type: from.into(),
            to_type: to.into(),
            message: msg.into(),
        }
    }
    
    /// 创建类未找到异常
    pub fn class_not_found(class_name: impl Into<String>, msg: impl Into<String>) -> Self {
        Exception::ClassNotFoundException {
            class_name: class_name.into(),
            cause: None,
            message: msg.into(),
        }
    }
    
    /// 创建 IO 异常
    pub fn io_error(kind: IOErrorKind, msg: impl Into<String>, path: Option<String>) -> Self {
        Exception::IOException {
            kind,
            message: msg.into(),
            path,
        }
    }
}

impl JvmError {
    /// 创建内存溢出错误
    pub fn out_of_memory(heap_size: u64, requested: u64, msg: impl Into<String>) -> Self {
        JvmError::OutOfMemoryError {
            heap_size,
            requested,
            message: msg.into(),
        }
    }
    
    /// 创建栈溢出错误
    pub fn stack_overflow(thread_name: impl Into<String>, stack_depth: usize, msg: impl Into<String>) -> Self {
        JvmError::StackOverflowError {
            thread_name: thread_name.into(),
            stack_depth,
            message: msg.into(),
        }
    }
    
    /// 创建类格式错误
    pub fn class_format(class_name: impl Into<String>, msg: impl Into<String>, offset: Option<u32>) -> Self {
        JvmError::ClassFormatError {
            class_name: class_name.into(),
            message: msg.into(),
        }
    }
    
    /// 创建类未找到定义错误
    pub fn no_class_def_found(class_name: impl Into<String>, msg: impl Into<String>, cause: Option<String>) -> Self {
        JvmError::NoClassDefFoundError {
            class_name: class_name.into(),
            cause,
            message: msg.into(),
        }
    }
    
    /// 创建方法未找到错误
    pub fn no_such_method(class_name: impl Into<String>, method_name: impl Into<String>, msg: impl Into<String>) -> Self {
        JvmError::NoSuchMethodError {
            class_name: class_name.into(),
            method_name: method_name.into(),
            method_descriptor: None,
            message: msg.into(),
        }
    }
    
    /// 创建字段未找到错误
    pub fn no_such_field(class_name: impl Into<String>, field_name: impl Into<String>, msg: impl Into<String>) -> Self {
        JvmError::NoSuchFieldError {
            class_name: class_name.into(),
            field_name: field_name.into(),
            field_descriptor: None,
            message: msg.into(),
        }
    }
}

// ========== 单元测试 ==========

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_runtime_exceptions() {
        let npe = Exception::null_pointer("Cannot invoke method on null");
        assert!(npe.is_runtime_exception());
        assert!(!npe.is_checked_exception());
        assert_eq!(npe.type_name(), "NullPointerException");
        
        let aob = Exception::array_index_out_of_bounds(5, 3, "Index out of bounds");
        assert!(aob.is_runtime_exception());
        assert_eq!(
            aob.message(),
            "Index out of bounds (index=5, length=3)"
        );
        
        let cast = Exception::class_cast("String", "Integer", "Cannot cast");
        assert!(cast.is_runtime_exception());
        assert_eq!(
            cast.message(),
            "Cannot cast (from=String, to=Integer)"
        );
    }
    
    #[test]
    fn test_checked_exceptions() {
        let io_err = Exception::io_error(
            IOErrorKind::NotFound,
            "File not found",
            Some("/tmp/test.txt".to_string())
        );
        assert!(!io_err.is_runtime_exception());
        assert!(io_err.is_checked_exception());
        assert_eq!(
            io_err.message(),
            "File not found: File not found at '/tmp/test.txt'"
        );
        
        let cnf = Exception::class_not_found("com.example.Test", "Class missing");
        assert!(!cnf.is_runtime_exception());
        assert!(cnf.is_checked_exception());
    }
    
    #[test]
    fn test_errors() {
        let oom = JvmError::out_of_memory(1024*1024*1024, 512*1024*1024, "Java heap space");
        assert!(oom.is_virtual_machine_error());
        assert!(!oom.is_linkage_error());
        assert_eq!(
            oom.message(),
            "Java heap space (heap=1073741824, requested=536870912)"
        );
        
        let format_err = JvmError::class_format(
            "com.example.Test",
            "Invalid magic number",
            Some(0)
        );
        assert!(!format_err.is_virtual_machine_error());
        assert!(format_err.is_linkage_error());
        assert_eq!(
            format_err.message(),
            "Invalid magic number (class=com.example.Test, offset=0)"
        );
        
        let method_err = JvmError::no_such_method(
            "com.example.Test",
            "run",
            "Method not found"
        );
        assert!(method_err.is_linkage_error());
    }
    
    #[test]
    fn test_throwable_conversions() {
        let exception: Throwable = Exception::null_pointer("test").into();
        assert!(!exception.is_error());
        assert!(exception.is_runtime_exception());
        
        let error: Throwable = JvmError::class_format("Test", "error", None).into();
        assert!(error.is_error());
        assert!(error.is_linkage_error());
    }
}