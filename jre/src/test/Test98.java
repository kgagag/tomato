package test;


import java.io.IOException;
import java.net.ServerSocket;

public class Test98 {
    public void test() throws IOException {
        ServerSocket serverSocket = new ServerSocket(8080);
        try {
            serverSocket.accept();
        } catch (IOException e) {
            throw new RuntimeException(e);
        }

    }

    public static void main(String[] args) throws IOException {
        Test98 test98 = new Test98();
        test98.test();
    }
}
