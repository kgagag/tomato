package test;

import java.net.InetAddress;
import java.net.UnknownHostException;

public class InetAddressTest {
    public void test() throws UnknownHostException {
        InetAddress inetAddress = InetAddress.getLocalHost();
        System.out.println(inetAddress.getHostAddress());
    }

    public static void main(String[] args) throws UnknownHostException {
        InetAddressTest inetAddressTest = new InetAddressTest();
        inetAddressTest.test();
    }
}
