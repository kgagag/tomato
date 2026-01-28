package tomato.test;

public class Test63 {
    public int test(){
        //int i =   Float.floatToRawIntBits(123.01f);
        //System.out.println(i);
        float f = Float.intBitsToFloat(1123419423);
         return f == 123.01f ? Result.SUCCESS : Result.FAILED;
    }

    public static void main(String[] args) {
        new Test63().test();
    }
}
