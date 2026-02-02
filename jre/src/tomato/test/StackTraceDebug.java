package tomato.test;

import java.lang.reflect.Field;

public class StackTraceDebug {
    public static void main(String[] args) throws Exception {
        Exception e = new Exception("测试");
        
        // 查看初始状态
        printStackTraceState(e, "创建后");
        
        // 调用 fillInStackTrace
        e.fillInStackTrace();
        printStackTraceState(e, "fillInStackTrace 后");
        
        // 获取堆栈（触发计算）
        e.getStackTrace();
        printStackTraceState(e, "getStackTrace 后");
    }
    
    static void printStackTraceState(Exception e, String stage) throws Exception {
        System.out.println("\n=== " + stage + " ===");
        
        Field stackTraceField = Throwable.class.getDeclaredField("stackTrace");
        stackTraceField.setAccessible(true);
        StackTraceElement[] stackTrace = (StackTraceElement[]) stackTraceField.get(e);
        
//        Field backtraceField = Throwable.class.getDeclaredField("backtrace");
//        backtraceField.setAccessible(true);
//        Object backtrace = backtraceField.get(e);
        
//        System.out.println("stackTrace: " +
//            (stackTrace == Throwable.UNASSIGNED_STACK ? "UNASSIGNED_STACK" :
//             stackTrace == null ? "null" : "已计算(" + stackTrace.length + "个元素)"));
      System.out.println("stackTrace: " + stackTrace.length);
    }
}