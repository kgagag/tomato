package test;

public class LookupSwitchExample {
    public  int test() {
       return getValue('z') == 2600 ? Result.SUCCESS:Result.FAILED;
    }

    public static int getValue(char character) {
        switch (character) {
            case 'a': return 100;
            case 'f': return 600;
            case 'k': return 1100;
            case 'p': return 1600;
            case 'u': return 2100;
            case 'z': return 2600;
            default: return -1; // 对于非指定字符返回 -1
        }
    }
}
