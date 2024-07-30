package test;

import java.io.FileDescriptor;
import java.io.FileInputStream;
import java.io.FileOutputStream;
import java.io.IOException;

public class FileDescriptorExample {
    public static void main(String[] args) {
        try {
            // 打开一个文件输入流
            FileInputStream fis = new FileInputStream("e:/huan_erp.sql");
            // 获取文件描述符
            FileDescriptor fd = fis.getFD();
            System.out.println("File descriptor for input.txt: " + fd);

            // 打开一个文件输出流
            FileOutputStream fos = new FileOutputStream("output.txt");
            // 获取文件描述符
            FileDescriptor fdOut = fos.getFD();
            System.out.println("File descriptor for output.txt: " + fdOut);

            // 使用标准输出的文件描述符
            FileOutputStream stdout = new FileOutputStream(FileDescriptor.out);
            stdout.write("Hello, World!\n".getBytes());
            stdout.flush();
        } catch (IOException e) {
            e.printStackTrace();
        }
    }
}
