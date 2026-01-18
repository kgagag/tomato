package test;

public class TestPrimitives {
    // 1. 基本类型与包装类转换
    public int testBoolean() {
        Boolean b = Boolean.valueOf(true);
        return b == true ? Result.SUCCESS : Result.FAILED;
    }
    
    public int testBooleanFalse() {
        Boolean b = Boolean.valueOf(false);
        return !b ? Result.SUCCESS : Result.FAILED;
    }
    
    public int testByte() {
        Byte b = Byte.valueOf((byte)127);
        return b == 127 ? Result.SUCCESS : Result.FAILED;
    }
    
    public int testByteMin() {
        Byte b = Byte.valueOf(Byte.MIN_VALUE);
        return b == -128 ? Result.SUCCESS : Result.FAILED;
    }
    
    public int testShort() {
        Short s = Short.valueOf((short)32767);
        return s == 32767 ? Result.SUCCESS : Result.FAILED;
    }
    
    public int testShortNegative() {
        Short s = Short.valueOf((short)-32768);
        return s == -32768 ? Result.SUCCESS : Result.FAILED;
    }
    
    public int testInteger() {
        Integer i = Integer.valueOf(2147483647);
        return i == 2147483647 ? Result.SUCCESS : Result.FAILED;
    }
    
    public int testIntegerMin() {
        Integer i = Integer.valueOf(Integer.MIN_VALUE);
        return i == -2147483648 ? Result.SUCCESS : Result.FAILED;
    }
    
    public int testLong() {
        Long l = Long.valueOf(9223372036854775807L);
        return l == 9223372036854775807L ? Result.SUCCESS : Result.FAILED;
    }
    
    public int testLongNegative() {
        Long l = Long.valueOf(-9223372036854775808L);
        return l == -9223372036854775808L ? Result.SUCCESS : Result.FAILED;
    }
    
    public int testFloat() {
        Float f = Float.valueOf(3.14f);
        return f == 3.14f ? Result.SUCCESS : Result.FAILED;
    }
    
    public int testFloatNaN() {
        Float f = Float.valueOf(Float.NaN);
        return Float.isNaN(f) ? Result.SUCCESS : Result.FAILED;
    }
    
    public int testFloatInfinity() {
        Float f = Float.valueOf(Float.POSITIVE_INFINITY);
        return f == Float.POSITIVE_INFINITY ? Result.SUCCESS : Result.FAILED;
    }
    
    // 2. 浮点数精度和比较
    public int testDoublePrecision() {
        double d1 = 0.1;
        double d2 = 0.2;
        double d3 = 0.3;
        return (d1 + d2) == d3 ? Result.FAILED : Result.SUCCESS; // 应该不相等
    }
    
    public int testFloatPrecision() {
        float f1 = 0.1f;
        float f2 = 0.2f;
        float f3 = 0.3f;
        return (f1 + f2) == f3 ? Result.FAILED : Result.SUCCESS; // 应该不相等
    }
    
    // 3. 包装类缓存测试（Integer缓存-128到127）
    public int testIntegerCache() {
        Integer a = Integer.valueOf(127);
        Integer b = Integer.valueOf(127);
        return a == b ? Result.SUCCESS : Result.FAILED; // 应该相等（同一对象）
    }
    
    public int testIntegerNoCache() {
        Integer a = Integer.valueOf(128);
        Integer b = Integer.valueOf(128);
        return a != b ? Result.SUCCESS : Result.FAILED; // 可能不相等（非缓存值）
    }
    
    // 4. 基本类型运算
    public int testIntArithmetic() {
        int a = 10;
        int b = 3;
        return (a + b == 13 && a - b == 7 && a * b == 30 && a / b == 3 && a % b == 1) 
                ? Result.SUCCESS : Result.FAILED;
    }
    
    public int testLongArithmetic() {
        long a = 10000000000L;
        long b = 3L;
        return (a + b == 10000000003L && a - b == 9999999997L && 
                a * b == 30000000000L && a / b == 3333333333L && a % b == 1L)
                ? Result.SUCCESS : Result.FAILED;
    }
    
    // 5. 类型转换
    public int testWideningConversion() {
        byte b = 100;
        short s = b;
        int i = s;
        long l = i;
        float f = l;
        double d = f;
        return d == 100.0 ? Result.SUCCESS : Result.FAILED;
    }
    
    public int testNarrowingConversion() {
        double d = 123.456;
        float f = (float)d;
        long l = (long)f;
        int i = (int)l;
        short s = (short)i;
        byte b = (byte)s;
        return b == 123 ? Result.SUCCESS : Result.FAILED;
    }
    
    // 6. 边界值测试
    public int testByteOverflow() {
        byte b = (byte)128; // 应该溢出为-128
        return b == -128 ? Result.SUCCESS : Result.FAILED;
    }
    
    public int testShortOverflow() {
        short s = (short)32768; // 应该溢出为-32768
        return s == -32768 ? Result.SUCCESS : Result.FAILED;
    }
    
    // 7. 比较操作
    public int testFloatComparison() {
        float f1 = 0.0f / 0.0f; // NaN
        float f2 = 0.0f / 0.0f; // NaN
        return (f1 != f2 && !(f1 == f2) && !(f1 < f2) && !(f1 > f2) && !(f1 <= f2) && !(f1 >= f2))
                ? Result.SUCCESS : Result.FAILED;
    }
    
    public int testDoubleComparison() {
        double d1 = 1.0;
        double d2 = 2.0;
        return (d1 < d2 && d1 <= d2 && d2 > d1 && d2 >= d1 && d1 != d2 && !(d1 == d2))
                ? Result.SUCCESS : Result.FAILED;
    }
    
    // 8. 特殊值测试
    public int testDoubleZero() {
        double posZero = 0.0;
        double negZero = -0.0;
        return (posZero == negZero && 1.0/posZero == Double.POSITIVE_INFINITY && 
                1.0/negZero == Double.NEGATIVE_INFINITY)
                ? Result.SUCCESS : Result.FAILED;
    }
    
    public int testFloatZero() {
        float posZero = 0.0f;
        float negZero = -0.0f;
        return (posZero == negZero && 1.0f/posZero == Float.POSITIVE_INFINITY && 
                1.0f/negZero == Float.NEGATIVE_INFINITY)
                ? Result.SUCCESS : Result.FAILED;
    }
    
    // 9. 字符串转换
    public int testParseInt() {
        int i = Integer.parseInt("12345");
        StringHelper.print20240503(i + "===");
        return i == 12345 ? Result.SUCCESS : Result.FAILED;
    }
    
    public int testParseDouble() {
        double d = Double.parseDouble("3.1415926");
        return Math.abs(d - 3.1415926) < 0.000001 ? Result.SUCCESS : Result.FAILED;
    }
    
    // 10. 混合类型运算
    public int testMixedArithmetic() {
        int i = 10;
        long l = 20L;
        float f = 30.0f;
        double d = 40.0;
        
        double result = i + l + f + d;
        return Math.abs(result - 100.0) < 0.000001 ? Result.SUCCESS : Result.FAILED;
    }
    
    // 11. 你的原始测试
    public int testDoubleValueOf() {
        double d = Double.valueOf(0.1d);
        return d == 0.1d ? Result.SUCCESS : Result.FAILED;
    }
}