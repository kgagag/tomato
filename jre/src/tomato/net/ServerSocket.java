package tomato.net;

import java.io.IOException;

public class ServerSocket {
    InetAddress bind;

    public ServerSocket(Integer port) {
        bind = new InetAddress(new short[]{127, 0, 0, 1}, port);
    }

    public Socket accept() throws IOException {
        Socket socket = new Socket(bind, 3000);
        socket.accept();
        return socket;
    }
}
