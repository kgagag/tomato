package test;

import java.util.Arrays;
import java.util.concurrent.TimeUnit;

public class Test88 {
    public int test(){
        if(MyTestUnit.assertEquals(new int[]{1,2},new int[]{1,2})){
            return Result.SUCCESS;
        }
        return Result.FAILED;
    }
}
