package test;

import java.nio.charset.Charset;

public class Test99 {
    public int test() {
        String a = "特懒狗 123 哈哈 한국어";
        byte[] bytes = a.getBytes();
        for (byte b : bytes) {
            StringHelper.print20240503("" + b);
        }
        String b = new String(bytes);
        StringHelper.print20240503(b);
        return Result.SUCCESS;
    }

    public static void main(String[] args) {
//        Test99 test99 = new Test99();
//        System.out.println(test99.test());
//        Charset defaultCharset = Charset.defaultCharset();
//        System.out.println("默认字符集: " + defaultCharset.name());

        String str = "默认字符集";
        //StringHelper.print20240503(str);
        byte[] bytes = str.getBytes();
        //StringHelper.print20240503(new String(bytes));
    }

}

