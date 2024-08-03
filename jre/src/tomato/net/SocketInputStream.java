package tomato.net;

import java.io.FileInputStream;
import java.io.FileNotFoundException;
import java.io.FileOutputStream;

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
