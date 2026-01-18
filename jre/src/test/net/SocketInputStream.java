package test.net;

import java.io.FileNotFoundException;

public class SocketInputStream {

    int fd;
    public SocketInputStream(String name,int fd) throws FileNotFoundException {
        this.fd = fd;
    }
    public void close(){
        close0();
    }

    private native void close0();
}
