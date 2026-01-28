package tomato.test;

public class Test61 {
    public int test(){
       int i =  Float.floatToRawIntBits(22.0f);
        float f = Float.intBitsToFloat(i);
        return f == 22.0f ? Result.SUCCESS:Result.FAILED;
    }

    public static void main(String[] args) {
        Test61 test61 = new Test61();
        test61.test();
    }
}
