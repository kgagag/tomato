package tomato.net;

import java.io.FileNotFoundException;
import java.io.FileOutputStream;
import java.io.IOException;

public class SocketOutputStream extends FileOutputStream {
    public SocketOutputStream(String name) throws FileNotFoundException {
        super(name);
    }

    @Override
    public void write(byte[] b){
        write0(b);
    }

    private native void  write0(byte[] b);
}
