package tomato.test;

public class Test52 {
    public void test(){
        char[] c = new char[]{'h','e','l','l','o'};
        char[] chars = new char[5];
        System.arraycopy(c, 0, chars, 0,
                Math.min(2, 4));
        StringHelper.print20240503(new String(chars));

    }

    public static void main(String[] args) {
        Test52 test50 = new Test52();
        test50.test();
    }
}
