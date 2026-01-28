package tomato.test;

public class AlphabetSwitch {
    public  int test() {
        return getValue('e') == 500 ? Result.SUCCESS : Result.FAILED;
    }

    public static int getValue(char character) {
        switch (character) {
            case 'a': return 100;
            case 'b': return 200;
            case 'c': return 300;
            case 'd': return 400;
            case 'e': return 500;
            case 'f': return 600;
            case 'g': return 700;
            case 'h': return 800;
            case 'i': return 900;
            case 'j': return 1000;
            case 'k': return 1100;
            case 'l': return 1200;
            case 'm': return 1300;
            case 'n': return 1400;
            case 'o': return 1500;
            case 'p': return 1600;
            case 'q': return 1700;
            case 'r': return 1800;
            case 's': return 1900;
            case 't': return 2000;
            case 'u': return 2100;
            case 'v': return 2200;
            case 'w': return 2300;
            case 'x': return 2400;
            case 'y': return 2500;
            case 'z': return 2600;
            default: return -1; // 对于非字母字符返回 -1
        }
    }
}
