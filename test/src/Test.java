public class Test {
    public static void main(String[] args) throws Exception{
//
       //  两个int相加
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

        InterfaceTest interfaceTest = new Test22();
        interfaceTest.test(20240320,5);
        new Test23().test();

        //测试数组作为参数
        new Test24().test(new int[20240320],new long[5]);
        new Test25().test(new int[]{0,20240320},new int[]{0,0,5});
        // 减法
        new Test26().test(20240325,0);

        new Test27().test(new int[100][100],new int[50][50]);

        new Test28().test(new int[100][100],new int[50][50]);

        //测试二维数组作为参数
        new Test29().test(new int[][]{{1}},new int[][]{{2}});
    }
}
