package test;

import java.util.HashMap;
import java.util.Map;

public class TestMap {
    public int test(){
        HashMap<String,String> map = new HashMap<>();
        map.put("name","zhangsan");
        StringHelper.print20240503(map.get("name"));
        return 20240325;
    }
}
