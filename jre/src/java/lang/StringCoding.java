package java.lang;

import java.io.UnsupportedEncodingException;
import java.nio.charset.Charset;

public class StringCoding {
    static  byte[] encode(char[] ca, int off, int len)  {
        return encode("utf8",ca,off,len);
    }

    static  byte[] encode(Charset cs, char[] ca, int off, int len)  {
        return encode(cs.name(),ca,off,len);
    }

    static  char[] decode(Charset charset, byte[] ba, int off, int len) {
        return decode0(ba,off,len);
    }

    static  char[] decode( byte[] ba, int off, int len) {
        return decode0(ba,off,len);
    }


    static  char[] decode(String charsetName, byte[] ba, int off, int len) {
        return decode0(ba,off,len);
    }

    static  byte[] encode(String charsetName, char[] ca, int off, int len) {
        return encode0(ca,off,len);
    }

    /**
     *  encode charset utf8
     */

    static native byte[] encode0(char[] ca, int off, int len) ;

    /**
     *  decode charset utf8
     */
    static native char[] decode0(byte[] ba, int off, int len);
}
