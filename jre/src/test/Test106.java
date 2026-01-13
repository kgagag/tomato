package test;

import java.util.HashMap;

public class Test106 {

    public int test() {
        String a = null;
        if (a instanceof  String) {
            return Result.SUCCESS;
        }
        return Result.FAILED;
    }
    public static void main(String[] args) {
        Test106 test106 = new Test106();
        test106.test();
    }
}
