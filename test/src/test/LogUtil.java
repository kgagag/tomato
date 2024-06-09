package test;

import java.io.FileOutputStream;

public class LogUtil{
   static void log(String msg) throws Exception{
           FileOutputStream outputStream = new FileOutputStream("e:/logs/log.log",true);
            byte[] bytes = msg.getBytes();
            outputStream.write(bytes,0,bytes.length);
    }
}