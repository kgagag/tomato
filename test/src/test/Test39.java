package test;

import java.io.FileDescriptor;
import java.io.FileOutputStream;
import java.io.PrintStream;

public class Test39 {
    public void test(){
        StringHelper.print20240503("输出一段中文和ying wen");
    }
    public static void main(String[] args) {
        String str = "hello world";
        FileOutputStream fdOut = new FileOutputStream(FileDescriptor.out);
        PrintStream printStream = new PrintStream(fdOut);
        printStream.print(str);
    }
}
