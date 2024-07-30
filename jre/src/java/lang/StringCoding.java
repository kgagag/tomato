package java.lang;

import java.io.UnsupportedEncodingException;
import java.nio.charset.Charset;

public class StringCoding {
    static  byte[] encode(char[] ca, int off, int len) throws UnsupportedEncodingException {
        return encode("utf8",ca,off,len);
    }

    static  byte[] encode(Charset cs, char[] ca, int off, int len) throws UnsupportedEncodingException {
        return encode(cs.name(),ca,off,len);
    }

    static native char[] decode(String charsetName, byte[] ba, int off, int len)
            throws UnsupportedEncodingException;

    static native byte[] encode(String charsetName, char[] ca, int off, int len)
            throws UnsupportedEncodingException;


}
