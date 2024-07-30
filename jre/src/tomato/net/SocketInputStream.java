package tomato.net;

import java.io.FileInputStream;
import java.io.FileNotFoundException;
import java.io.FileOutputStream;

public class SocketInputStream extends FileInputStream {

    public SocketInputStream(String name) throws FileNotFoundException {
        super(name);
    }
}
