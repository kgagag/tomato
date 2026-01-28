package tomato.test.net;

import java.io.FileNotFoundException;
import java.io.IOException;

public class SocketOutputStream  {
    int fd;

    public SocketOutputStream(String name,int fd) throws FileNotFoundException {
        this.fd = fd;
    }
    public void write(byte[] b) throws IOException{
        write0(b);
    }

    private native void  write0(byte[] b);

    public void close(){
        close0();
    }

    private native void close0();
}
