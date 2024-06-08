public class Test63 {
    public int test(){
        //int i =   Float.floatToRawIntBits(123.01f);
        //System.out.println(i);
        float f = Float.intBitsToFloat(1123419423);
         return f == 123.01f ? 20240325 : 20240324;
    }

    public static void main(String[] args) {
        new Test63().test();
    }
}
