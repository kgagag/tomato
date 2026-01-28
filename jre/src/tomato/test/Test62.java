package tomato.test;

public class Test62 {
    public int test(){
      int i =   Float.floatToRawIntBits(123.01f);
        //System.out.println(i);
        return 1123419423 == i ? Result.SUCCESS : Result.FAILED;
    }

    public static void main(String[] args) {
        new Test62().test();
    }
}
