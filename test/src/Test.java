import test.*;

import java.io.IOException;

public class Test {
    public static void main(String[] args) throws IOException {
        //
        //  两个int相加
        new Test1().test();
        // 两个 float 相加
        new Test2().test();
        //  两个高精度 double 相加
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
//
        //测试递归
        Test10.test();

        //测试 invokestatic 和 putstatic 指令
        Test11.test();

        //测试 invokevirtual

        Test12.test();

        // 测试 getfield 指令
        Test13.test();

        // 测试一维数组赋值读取
        new Test14().test();

        //   测试多维数组赋值读取
        new Test15().test();

        // new test.Test16().test();
        // 测试计算数组长度
        new Test18().test();
        // 一维数组循环赋值
        new Test19().test();
        // 多维数组循环赋值
        new Test20().test();

        // 测试 invikeinterface
        InterfaceTest interfaceTest = new Test22();
        interfaceTest.test(20240320, 5);
        new Test23().test();

        //测试数组作为参数
        new Test24().test(new int[20240320], new long[5]);
        new Test25().test(new int[]{0, 20240320}, new int[]{0, 0, 5});

        // 减法
        new Test26().test(20240325, 0);

        //测试二维数组作为参数
        new Test27().test(new int[100][100], new int[50][50]);
        new Test28().test(new int[100][100], new int[50][50]);
        new Test29().test(new int[][]{{1}}, new int[][]{{2}});

        //普通继承
        new Test31().test();

        // 先实现接口后继承
        new Test33().test(20240320, 5);

        //继承抽象类
        new Test34().test(20240320, 5);

        // 力扣 第33 题
        new LeetCode_33().test();

        //测试多种数据类型入参
        new Test35().test('1', 3, 4, 5.0f, 6d);

        // 力扣 第31 题
        new LeetCode_31().test();

        //创建String 对象
        new Test37().test();
        new Test39().test();
        new Test42().test();
        new Test40().test();
        new Test44().test();
        new Test45().test();
        //new test.Test46().test();
        new Test47().test();

        new Test49().test();
//
        new Test50().test();
////
        new Test51().test();
////
////
        new Test52().test();
//
//
        new Test53().test();

        new Test55().test();

        new Test56().test();

        new Test57().test();

        new Test58().test();

        new Test59().test();

        new Test60().test();
        new Test61().test();
        new Test62().test();
        new Test63().test();

        new Test64().test();

        new Test65().test();

        //测试 new String
        new Test66().test();

        //测试substring
        new Test67().test();
        // 测试 String concat
        new Test69().test();

        //测试 "+" 拼接字符串
        new Test70().test();
//
//        // 测试GC
        new TestGc().test();
        new TestGC2().test();
//
//       // new test.Test71().test();
//        // 加载Throwable
//        new test.Test73().test();
//        new test.Test75().test();
//
        new Test78().test();
        new Test79().test();
        new Test80().test();
        new Test81().test();
        //测试 String.valueOf()
        new Test82().test();
        Test83.test();

        new Test85().test();
        new Test86().test();
        //测试 hashMap
        new TestMap().test();
        // 测试 hashSet
        new TestSet().test();
        // 测试 ArrayList
        new TestList().test();
        new TestList2().test();
    }
}
