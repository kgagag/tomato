package tomato.test;

public class SimpleBasicTypeTestRunner {
    public  void test() {
        TestPrimitives test = new TestPrimitives();
        
        StringHelper.print20240503("=== 开始执行基本类型测试 ===");
        
        // 逐个测试
        runTest("testBoolean", test.testBoolean());
        runTest("testBooleanFalse", test.testBooleanFalse());
        runTest("testByte", test.testByte());
        runTest("testByteMin", test.testByteMin());
        runTest("testShort", test.testShort());
        runTest("testShortNegative", test.testShortNegative());
        runTest("testInteger", test.testInteger());
        runTest("testIntegerMin", test.testIntegerMin());
        runTest("testLong", test.testLong());
        runTest("testLongNegative", test.testLongNegative());
        runTest("testFloat", test.testFloat());
        runTest("testFloatNaN", test.testFloatNaN());
        runTest("testFloatInfinity", test.testFloatInfinity());
        runTest("testDoublePrecision", test.testDoublePrecision());
        runTest("testFloatPrecision", test.testFloatPrecision());
        runTest("testIntegerCache", test.testIntegerCache());
        runTest("testIntegerNoCache", test.testIntegerNoCache());
        runTest("testIntArithmetic", test.testIntArithmetic());
        runTest("testLongArithmetic", test.testLongArithmetic());
        runTest("testWideningConversion", test.testWideningConversion());
        runTest("testNarrowingConversion", test.testNarrowingConversion());
        runTest("testByteOverflow", test.testByteOverflow());
        runTest("testShortOverflow", test.testShortOverflow());
        runTest("testFloatComparison", test.testFloatComparison());
        runTest("testDoubleComparison", test.testDoubleComparison());
        runTest("testDoubleZero", test.testDoubleZero());
        runTest("testFloatZero", test.testFloatZero());
        runTest("testParseInt", test.testParseInt());
//        runTest("testParseDouble", test.testParseDouble());
        runTest("testMixedArithmetic", test.testMixedArithmetic());
        runTest("testDoubleValueOf", test.testDoubleValueOf());
        
        StringHelper.print20240503("\n=== 测试完成 ===");
    }
    
    private static void runTest(String testName, int result) {
        if (result == Result.SUCCESS) {
            StringHelper.print20240503("✓ " + testName + " - 通过");
        } else {
            StringHelper.print20240503("✗ " + testName + " - 失败 (返回码: " + result + ")");
        }
    }

    public static void main(String[] args) {
//        SimpleBasicTypeTestRunner runner = new SimpleBasicTypeTestRunner();
//        runner.tomato.test();
//          StringHelper.print20240503(   Integer.parseInt("2", 10) + "");
        //Character.digit('2', 10);

         String A_DATA =
                "\u4800\u100F\u4800\u100F\u4800\u100F\u4800\u100F\u4800\u100F\u4800\u100F\u4800"+
                        "\u100F\u4800\u100F\u4800\u100F\u5800\u400F\u5000\u400F\u5800\u400F\u6000\u400F"+
                        "\u5000\u400F\u4800\u100F\u4800\u100F\u4800\u100F\u4800\u100F\u4800\u100F\u4800"+
                        "\u100F\u4800\u100F\u4800\u100F\u4800\u100F\u4800\u100F\u4800\u100F\u4800\u100F"+
                        "\u4800\u100F\u4800\u100F\u5000\u400F\u5000\u400F\u5000\u400F\u5800\u400F\u6000"+
                        "\u400C\u6800\030\u6800\030\u2800\030\u2800\u601A\u2800\030\u6800\030\u6800"+
                        "\030\uE800\025\uE800\026\u6800\030\u2000\031\u3800\030\u2000\024\u3800\030"+
                        "\u3800\030\u1800\u3609\u1800\u3609\u1800\u3609\u1800\u3609\u1800\u3609\u1800"+
                        "\u3609\u1800\u3609\u1800\u3609\u1800\u3609\u1800\u3609\u3800\030\u6800\030"+
                        "\uE800\031\u6800\031\uE800\031\u6800\030\u6800\030\202\u7FE1\202\u7FE1\202"+
                        "\u7FE1\202\u7FE1\202\u7FE1\202\u7FE1\202\u7FE1\202\u7FE1\202\u7FE1\202\u7FE1"+
                        "\202\u7FE1\202\u7FE1\202\u7FE1\202\u7FE1\202\u7FE1\202\u7FE1\202\u7FE1\202"+
                        "\u7FE1\202\u7FE1\202\u7FE1\202\u7FE1\202\u7FE1\202\u7FE1\202\u7FE1\202\u7FE1"+
                        "\202\u7FE1\uE800\025\u6800\030\uE800\026\u6800\033\u6800\u5017\u6800\033\201"+
                        "\u7FE2\201\u7FE2\201\u7FE2\201\u7FE2\201\u7FE2\201\u7FE2\201\u7FE2\201\u7FE2"+
                        "\201\u7FE2\201\u7FE2\201\u7FE2\201\u7FE2\201\u7FE2\201\u7FE2\201\u7FE2\201"+
                        "\u7FE2\201\u7FE2\201\u7FE2\201\u7FE2\201\u7FE2\201\u7FE2\201\u7FE2\201\u7FE2"+
                        "\201\u7FE2\201\u7FE2\201\u7FE2\uE800\025\u6800\031\uE800\026\u6800\031\u4800"+
                        "\u100F\u4800\u100F\u4800\u100F\u4800\u100F\u4800\u100F\u4800\u100F\u5000\u100F"+
                        "\u4800\u100F\u4800\u100F\u4800\u100F\u4800\u100F\u4800\u100F\u4800\u100F\u4800"+
                        "\u100F\u4800\u100F\u4800\u100F\u4800\u100F\u4800\u100F\u4800\u100F\u4800\u100F"+
                        "\u4800\u100F\u4800\u100F\u4800\u100F\u4800\u100F\u4800\u100F\u4800\u100F\u4800"+
                        "\u100F\u4800\u100F\u4800\u100F\u4800\u100F\u4800\u100F\u4800\u100F\u4800\u100F"+
                        "\u3800\014\u6800\030\u2800\u601A\u2800\u601A\u2800\u601A\u2800\u601A\u6800"+
                        "\034\u6800\030\u6800\033\u6800\034\000\u7005\uE800\035\u6800\031\u4800\u1010"+
                        "\u6800\034\u6800\033\u2800\034\u2800\031\u1800\u060B\u1800\u060B\u6800\033"+
                        "\u07FD\u7002\u6800\030\u6800\030\u6800\033\u1800\u050B\000\u7005\uE800\036"+
                        "\u6800\u080B\u6800\u080B\u6800\u080B\u6800\030\202\u7001\202\u7001\202\u7001"+
                        "\202\u7001\202\u7001\202\u7001\202\u7001\202\u7001\202\u7001\202\u7001\202"+
                        "\u7001\202\u7001\202\u7001\202\u7001\202\u7001\202\u7001\202\u7001\202\u7001"+
                        "\202\u7001\202\u7001\202\u7001\202\u7001\202\u7001\u6800\031\202\u7001\202"+
                        "\u7001\202\u7001\202\u7001\202\u7001\202\u7001\202\u7001\u07FD\u7002\201\u7002"+
                        "\201\u7002\201\u7002\201\u7002\201\u7002\201\u7002\201\u7002\201\u7002\201"+
                        "\u7002\201\u7002\201\u7002\201\u7002\201\u7002\201\u7002\201\u7002\201\u7002"+
                        "\201\u7002\201\u7002\201\u7002\201\u7002\201\u7002\201\u7002\201\u7002\u6800"+
                        "\031\201\u7002\201\u7002\201\u7002\201\u7002\201\u7002\201\u7002\201\u7002"+
                        "\u061D\u7002";
        char[] array =  A_DATA.toCharArray() ;
        StringHelper.print20240503("len===="+array.length);
    }
}