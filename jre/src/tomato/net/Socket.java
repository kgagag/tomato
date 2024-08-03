package tomato.net;

import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;

public  class Socket {
    private InetAddress bind;

    SocketOutputStream outputStream;

    public SocketOutputStream getOutputStream() {
        return outputStream;
    }

    SocketInputStream  inputStream;

    public SocketInputStream getInputStream() {
        return inputStream;
    }

    public void setInputStream(SocketInputStream inputStream) {
        this.inputStream = inputStream;
    }
    private int keepLive;

    public Socket(InetAddress bind, int keepLive) throws IOException {
        this.bind = bind;
        this.keepLive = keepLive;
        outputStream = new SocketOutputStream(null,this.hashCode());
        inputStream = new SocketInputStream(null,this.hashCode());
    }

    public InetAddress getBind() {
        return bind;
    }

    public void accept(){
         accept0(this);
    };
    private native void accept0(Socket socket);

    public void close(){
        this.outputStream.close();
        this.inputStream.close();
    }
}
