package test;

import java.util.HashMap;
import java.util.Map;

public class TestMap2 {
    public int test(){
        Map<Integer,Integer> map2 = new HashMap<>();
        map2.put(100,2222);
        Object o =   map2.get(100);
        if(o != null){
            return Result.SUCCESS;
        }
        return Result.FAILED;
    }

}
