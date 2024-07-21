package test;

import java.io.IOException;
import java.net.InetAddress;
import java.net.UnknownHostException;

public class InetAddressTest {
    public void test() throws IOException {
//        InetAddress inetAddress = InetAddress.getLocalHost();
//        InetAddress address = InetAddress.getByName("www.evimed.com");
//        System.out.println("IP address: " + address);
        InetAddress[] addresses = InetAddress.getAllByName("www.evimed.com");
        for (InetAddress addr : addresses) {
            System.out.println("IP address: " + addr);
        }

        InetAddress address = InetAddress.getByName("www.example.com");
        boolean reachable = address.isReachable(5000);  // 超时时间为 5000 毫秒
        System.out.println("Reachable: " + reachable);


//        System.out.println(inetAddress.getHostAddress());
    }

    public static void main(String[] args) throws IOException {
        InetAddressTest inetAddressTest = new InetAddressTest();
        inetAddressTest.test();
    }
}
