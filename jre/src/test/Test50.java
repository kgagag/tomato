package test;

public class Test50 {
    public void test(){
        char[] c = new char[]{'h','e','l','l','o'};
        String s = new String(c,0,2);
        StringHelper.print20240503(s);
    }

    public static void main(String[] args) {
        Test50 test50 = new Test50();
        test50.test();
    }
}
