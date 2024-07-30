package test.net;

/**
 * SimpleNetServer
 */
public class SimpleNetServer {
    public native byte[] listen();
    private int port;
    public native void handleClientRequest();
}
