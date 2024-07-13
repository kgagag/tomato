package test;

import java.io.FileOutputStream;
import java.io.IOException;

public class Test96 {
    public int test() throws IOException {
        FileOutputStream fileOutputStream = new FileOutputStream("e:/log.log");
        fileOutputStream.write("哈哈哈".getBytes(),0,"哈哈哈".getBytes().length);
        return Result.SUCCESS;
    }

    public static void main(String[] args) throws IOException {
        Test96 test96 = new Test96();
        test96.test();
    }
}
