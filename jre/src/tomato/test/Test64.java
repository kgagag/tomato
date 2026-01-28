package tomato.test;

import java.util.Arrays;

public class Test64 {
    public int test(){
        int[] ch = new int[]{Result.SUCCESS};
        int[] ints = Arrays.copyOf(ch, ch.length);
        return ints[0];
    }
}
