package test;

import java.util.HashMap;
import java.util.Map;

public class TestMap3 {
    public int test(){
        Map<Integer,Integer> map2 = new HashMap<>();
        map2.put(100,Result.SUCCESS);
        return map2.get(100);
    }

}
