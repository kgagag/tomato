package test;

public class Test83 {
    static void getChars(int i, int index, char[] buf) {
        int q, r;
        int charPos = index;
        char[] digits = {
                '0' , '1' , '2' , '3' , '4' , '5' ,
                '6' , '7' , '8' , '9' , 'a' , 'b' ,
                'c' , 'd' , 'e' , 'f' , 'g' , 'h' ,
                'i' , 'j' , 'k' , 'l' , 'm' , 'n' ,
                'o' , 'p' , 'q' , 'r' , 's' , 't' ,
                'u' , 'v' , 'w' , 'x' , 'y' , 'z'
        };
        while (true) {
           // StringHelper.print20240503("=============");
            q = (i * 52429) >>> (16+3);
            r = i - ((q << 3) + (q << 1));  // r = i-(q*10) ...
            buf [--charPos] = digits [r];
            i = q;
            if (i == 0) break;
        }

    }

    public static int test() {
        char[] ch = new char[3];
        getChars(123,3,ch);
        StringHelper.print20240503(new String(ch));
        return 20240325;
    }

    public static void main(String[] args) {
        test();
    }
}
