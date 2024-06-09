package test;

import java.io.File;
import java.io.IOException;

public class Test76 {
    public int test() throws IOException {
        File file = new File("e:/data20240519.txt");
        boolean f = file.createNewFile();
        return  f ? 20240325 : 20240324;
    }
}
