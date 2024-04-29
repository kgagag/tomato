public class Test {
    public static void main(String[] args) throws Exception{

     //    两个int相加
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

       Test10.test();
       Test11.test();
       Test12.test();
       Test13.test();

       // 测试一维数组赋值读取
        new Test14().test();
       //   测试多维数组赋值读取
        new Test15().test();

        // new Test16().test();
        // 测试计算数组长度
        new Test18().test();
        // 一维数组循环赋值
        new Test19().test();
        // 多维数组循环赋值
        new Test20().test();



    }
}
