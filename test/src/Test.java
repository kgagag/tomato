public class Test {
    public static void main(String[] args) throws Exception{
        // 两个int相加
        new Test1().test();
        // 两个 float 相加
        new Test2().test();
        // 两个double 相加
        new Test3().test();
        // for 循环
        new Test4().test();
        // while 循环
        new Test5().test();
        //递归修改成员变量
       new Test6().test();
        // 修改成员变量
        new Test7().test();
       // for 循环执行 100次 invokeVirtual
       new Test8().test();
          // 7层方法调用栈
       new Test9().test();
    //    // invokestatic putstatic getstatic
    //    Test11.test();
    //    // invokestatic
    //    Test12.test();
    //     Test13.test();
    }
}
