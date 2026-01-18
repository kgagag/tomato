package test;

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
        runTest("testParseDouble", test.testParseDouble());
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
        SimpleBasicTypeTestRunner runner = new SimpleBasicTypeTestRunner();
        runner.test();
    }
}