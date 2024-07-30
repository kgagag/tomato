package tomato.net;

import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;

public  class Socket {
    private InetAddress bind;

    private int keepLive;

    public Socket(InetAddress bind, int keepLive) {
        this.bind = bind;
        this.keepLive = keepLive;
    }

    public InetAddress getBind() {
        return bind;
    }

    public OutputStream getOutputStream() throws IOException{
        return new SocketOutputStream(null);
    }
    public InputStream getInputStream() throws IOException{
        return new SocketInputStream(null);
    }
    public void accept(){
         accept0(this);
    };
    private native void accept0(Socket socket);
}
