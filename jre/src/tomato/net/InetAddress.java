package tomato.net;


public class InetAddress {
    private short[] address;

    private int port;

    public short[] getAddress() {
        return address;
    }

    public Integer getPort() {
        return port;
    }

    public InetAddress(short[] address, Integer port){
        this.address = address;
        this.port = port;
    }

}
