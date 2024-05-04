import java.util.Arrays;

public class Test51 {
    public void test(){
        char[] c = new char[]{'h','e','l','l','o'};
        char[] chars = Arrays.copyOfRange(c, 0, 2);
        StringHelper.print20240503(new String(chars));
    }

    public static void main(String[] args) {
        Test51 test50 = new Test51();
        test50.test();
    }
}
