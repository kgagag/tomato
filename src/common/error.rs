/// JVM 可抛出异常/错误的统一枚举
/// 
/// 这是 Java 异常处理机制的 Rust 实现，将异常分为两大类：
/// - Exception: 程序可处理的异常（部分需要声明）
/// - Error: 严重系统错误，通常不应捕获
#[derive(Debug, Clone, PartialEq)]
pub enum Throwable {
    /// 程序异常（可恢复）
    Exception(Exception),
    /// 系统错误（不可恢复）
    Error(JvmError),
}

/// 程序异常类型
///
/// 对应 Java 的 Exception 类及其子类，分为：
/// - RuntimeException: 运行时异常，无需声明
/// - CheckedException: 检查异常，必须在方法签名中声明
#[derive(Debug, Clone, PartialEq)]
pub enum Exception {
    // ========== RuntimeException（运行时异常）==========
    
    /// 空指针异常
    /// 当应用程序试图在需要对象的地方使用 null 时抛出
    NullPointer(String),
    
    /// 数组索引越界异常
    /// 使用非法索引访问数组时抛出（索引为负或大于等于数组大小）
    ArrayIndexOutOfBounds {
        index: i32,
        length: i32,
        message: String,
    },
    
    /// 类转换异常
    /// 试图将对象强制转换为不是实例的子类时抛出
    ClassCast {
        from_type: String,
        to_type: String,
        message: String,
    },
    
    /// 算术异常
    /// 出现异常的算术条件时抛出（如除以零）
    Arithmetic(String),
    
    /// 非法参数异常
    /// 向方法传递非法或不合适的参数时抛出
    IllegalArgument(String),
    
    /// 非法状态异常
    /// 在非法或不适当的时间调用方法时抛出
    IllegalState(String),
    
    // ========== CheckedException（检查异常）==========
    
    /// IO异常
    /// 发生某种 I/O 异常时抛出的信号
    IOException {
        kind: IOErrorKind,
        message: String,
        path: Option<String>,
    },
    
    /// 类未找到异常
    /// 当应用程序试图加载类但找不到类定义时抛出
    ClassNotFound {
        class_name: String,
        message: String
    },

    FieldNotFound {
        class_name: String,
        field_name: String
    },
    
    /// 方法未找到异常
    /// 试图调用不存在的方法时抛出
    NoSuchMethod(String),
    
    /// 文件未找到异常
    /// 试图打开指定路径名表示的文件失败时抛出
    FileNotFound(String),
    
    /// 中断异常
    /// 当线程在等待、睡眠或其他方式占用时被中断时抛出
    Interrupted(String),
    
    /// 安全异常
    /// 由安全管理器抛出的异常，指示违反安全规定
    Security(String),
    
    /// 超时异常
    /// 阻塞操作超时时抛出
    Timeout(String),
    
    /// 解析异常
    /// 解析字符串或数据时发生错误
    Parse {
        target: String,
        position: usize,
        message: String,
    },
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
    Other,
}

impl std::fmt::Display for IOErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IOErrorKind::NotFound => write!(f, "NotFound"),
            IOErrorKind::PermissionDenied => write!(f, "PermissionDenied"),
            IOErrorKind::ConnectionRefused => write!(f, "ConnectionRefused"),
            IOErrorKind::ConnectionReset => write!(f, "ConnectionReset"),
            IOErrorKind::ConnectionAborted => write!(f, "ConnectionAborted"),
            IOErrorKind::NotConnected => write!(f, "NotConnected"),
            IOErrorKind::AddrInUse => write!(f, "AddrInUse"),
            IOErrorKind::AddrNotAvailable => write!(f, "AddrNotAvailable"),
            IOErrorKind::BrokenPipe => write!(f, "BrokenPipe"),
            IOErrorKind::AlreadyExists => write!(f, "AlreadyExists"),
            IOErrorKind::WouldBlock => write!(f, "WouldBlock"),
            IOErrorKind::InvalidInput => write!(f, "InvalidInput"),
            IOErrorKind::InvalidData => write!(f, "InvalidData"),
            IOErrorKind::TimedOut => write!(f, "TimedOut"),
            IOErrorKind::WriteZero => write!(f, "WriteZero"),
            IOErrorKind::Interrupted => write!(f, "Interrupted"),
            IOErrorKind::UnexpectedEof => write!(f, "UnexpectedEof"),
            IOErrorKind::Other => write!(f, "Other"),
        }
    }
}

/// 虚拟机错误类型
///
/// 对应 Java 的 Error 类及其子类，表示严重问题：
/// - 应用程序不应捕获这些错误
/// - 通常表示系统资源耗尽或 JVM 内部错误
#[derive(Debug, Clone, PartialEq)]
pub enum JvmError {
    // ========== VirtualMachineError（虚拟机错误）==========
    
    /// 内存溢出错误
    /// 当 JVM 无法分配对象所需内存，且垃圾收集器无法提供更多内存时抛出
    OutOfMemory {
        heap_size: u64,
        requested: u64,
        message: String,
    },
    
    /// 栈溢出错误
    /// 当应用程序递归太深而发生栈溢出时抛出
    StackOverflow {
        thread_name: String,
        stack_depth: usize,
        message: String,
    },
    
    /// 内部错误
    /// 发生 JVM 内部错误或资源限制时抛出
    InternalError(String),
    
    /// 未知错误
    /// 发生未知但严重的异常时抛出
    UnknownError(String),
    
    // ========== LinkageError（链接错误）==========
    
    /// 类定义未找到错误
    /// 当 JVM 或 ClassLoader 实例试图加载类定义但未找到时抛出
    NoClassDefFound {
        class_name: String,
        cause: Option<String>,
        message: String,
    },
    
    /// 不兼容的类更改错误
    /// 当某个类的定义发生不兼容更改时抛出
    IncompatibleClassChange {
        class_name: String,
        change_type: ClassChangeType,
        message: String,
    },
    
    /// 抽象方法错误
    /// 当应用程序试图调用抽象方法时抛出
    AbstractMethod(String),
    
    /// 不满足链接错误
    /// 当链接器无法解析符号引用时抛出
    UnsatisfiedLink {
        library_name: String,
        os_error: Option<String>,
        message: String,
    },
    
    /// 验证错误
    /// 当类文件验证失败时抛出
    Verify {
        class_name: String,
        bytecode_offset: u32,
        message: String,
    },
    
    // ========== Thread Error（线程错误）==========
    
    /// 线程死亡错误
    /// 当调用 Thread.stop 方法时抛出（已弃用）
    ThreadDeath(String),
    
    // ========== Assertion Error（断言错误）==========
    
    /// 断言错误
    /// 当断言语句失败时抛出
    Assertion {
        condition: String,
        file: Option<String>,
        line: Option<u32>,
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
    /// 更改类层次结构
    HierarchyChanged,
    /// 更改访问权限
    AccessChanged,
    /// 更改方法签名
    SignatureChanged,
    /// 更改类/接口状态
    ClassToInterface,
    InterfaceToClass,
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
}

impl Exception {
    /// 获取异常信息
    pub fn message(&self) -> String {
        match self {
            Exception::NullPointer(msg)
            | Exception::Arithmetic(msg)
            | Exception::IllegalArgument(msg)
            | Exception::IllegalState(msg)
            | Exception::NoSuchMethod(msg)
            | Exception::FileNotFound(msg)
            | Exception::Interrupted(msg)
            | Exception::Security(msg)
            | Exception::Timeout(msg) => msg.clone(),
            Exception::ArrayIndexOutOfBounds { index, length, message } => {
                format!("{} (index={}, length={})", message, index, length)
            }
            Exception::ClassCast { from_type, to_type, message } => {
                format!("{} (from={}, to={})", message, from_type, to_type)
            }
            Exception::IOException { kind, message, path } => {
                if let Some(p) = path {
                    format!("{}: {} at path: {}", kind, message, p)
                } else {
                    format!("{}: {}", kind, message)
                }
            }
            Exception::ClassNotFound { class_name, message } => {
                format!("{} (class={})", message, class_name)
            }
            Exception::Parse { target, position, message } => {
                format!("{} at position {} in '{}'", message, position, target)
            }
            Exception::FieldNotFound { class_name, field_name } => todo!(),
        }
    }
    
    /// 判断是否为运行时异常
    pub fn is_runtime_exception(&self) -> bool {
        matches!(
            self,
            Exception::NullPointer(_)
                | Exception::ArrayIndexOutOfBounds { .. }
                | Exception::ClassCast { .. }
                | Exception::Arithmetic(_)
                | Exception::IllegalArgument(_)
                | Exception::IllegalState(_)
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
            JvmError::OutOfMemory { heap_size, requested, message } => {
                format!("{} (heap={}, requested={})", message, heap_size, requested)
            }
            
            JvmError::StackOverflow { thread_name, stack_depth, message } => {
                format!("{} (thread={}, depth={})", message, thread_name, stack_depth)
            }
            
            JvmError::InternalError(msg)
            | JvmError::UnknownError(msg)
            | JvmError::AbstractMethod(msg)
            | JvmError::ThreadDeath(msg) => msg.clone(),
            
            JvmError::NoClassDefFound { class_name, cause, message } => {
                if let Some(c) = cause {
                    format!("{} (class={}, cause={})", message, class_name, c)
                } else {
                    format!("{} (class={})", message, class_name)
                }
            }
            
            JvmError::IncompatibleClassChange { class_name, change_type, message } => {
                format!("{} (class={}, change={:?})", message, class_name, change_type)
            }
            
            JvmError::UnsatisfiedLink { library_name, os_error, message } => {
                if let Some(e) = os_error {
                    format!("{} (library={}, os_error={})", message, library_name, e)
                } else {
                    format!("{} (library={})", message, library_name)
                }
            }
            
            JvmError::Verify { class_name, bytecode_offset, message } => {
                format!("{} (class={}, offset={})", message, class_name, bytecode_offset)
            }
            
            JvmError::Assertion { condition, file, line, message } => {
                let location = match (file, line) {
                    (Some(f), Some(l)) => format!(" at {}:{}", f, l),
                    (Some(f), None) => format!(" at {}", f),
                    (None, Some(l)) => format!(" at line {}", l),
                    (None, None) => String::new(),
                };
                format!("{} (condition={}){}", message, condition, location)
            }
        }
    }
    
    /// 判断是否为链接错误
    pub fn is_linkage_error(&self) -> bool {
        matches!(
            self,
            JvmError::NoClassDefFound { .. }
                | JvmError::IncompatibleClassChange { .. }
                | JvmError::AbstractMethod(_)
                | JvmError::UnsatisfiedLink { .. }
                | JvmError::Verify { .. }
        )
    }
    
    /// 判断是否为虚拟机错误
    pub fn is_virtual_machine_error(&self) -> bool {
        matches!(
            self,
            JvmError::OutOfMemory { .. }
                | JvmError::StackOverflow { .. }
                | JvmError::InternalError(_)
                | JvmError::UnknownError(_)
        )
    }
}

impl std::fmt::Display for Throwable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Throwable::Exception(e) => write!(f, "Exception: {}", e.message()),
            Throwable::Error(e) => write!(f, "Error: {}", e.message()),
        }
    }
}

impl std::fmt::Display for Exception {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl std::fmt::Display for JvmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl std::error::Error for Throwable {}
impl std::error::Error for Exception {}
impl std::error::Error for JvmError {}

// ========== 示例用法 ==========

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_exception_messages() {
        let npe = Exception::NullPointer("Attempt to invoke method on null object".to_string());
        assert!(npe.is_runtime_exception());
        assert!(!npe.is_checked_exception());
        assert_eq!(
            npe.message(),
            "Attempt to invoke method on null object"
        );
        
        let ioex = Exception::IOException {
            kind: IOErrorKind::NotFound,
            message: "File not found".to_string(),
            path: Some("/path/to/file.txt".to_string()),
        };
        assert!(!ioex.is_runtime_exception());
        assert!(ioex.is_checked_exception());
        assert_eq!(
            ioex.message(),
            "NotFound: File not found at path: /path/to/file.txt"
        );
    }
    
    #[test]
    fn test_error_messages() {
        let oom = JvmError::OutOfMemory {
            heap_size: 1024 * 1024 * 1024, // 1GB
            requested: 512 * 1024 * 1024,   // 512MB
            message: "Java heap space".to_string(),
        };
        assert!(oom.is_virtual_machine_error());
        assert!(!oom.is_linkage_error());
        assert_eq!(
            oom.message(),
            "Java heap space (heap=1073741824, requested=536870912)"
        );
        
        let linkage = JvmError::NoClassDefFound {
            class_name: "com.example.MissingClass".to_string(),
            cause: Some("ClassNotFoundException".to_string()),
            message: "Could not find class definition".to_string(),
        };
        assert!(!linkage.is_virtual_machine_error());
        assert!(linkage.is_linkage_error());
        assert_eq!(
            linkage.message(),
            "Could not find class definition (class=com.example.MissingClass, cause=ClassNotFoundException)"
        );
    }
    
    #[test]
    fn test_throwable_wrapper() {
        let throwable = Throwable::Exception(
            Exception::IllegalArgument("Invalid argument provided".to_string())
        );
        
        assert!(!throwable.is_error());
        assert!(throwable.is_runtime_exception());
        assert_eq!(
            throwable.message(),
            "Invalid argument provided"
        );
        
        let error = Throwable::Error(
            JvmError::StackOverflow {
                thread_name: "main".to_string(),
                stack_depth: 1024,
                message: "Recursive call too deep".to_string(),
            }
        );
        
        assert!(error.is_error());
        assert!(!error.is_runtime_exception());
        assert!(!error.is_checked_exception());
    }
}