package tomato.test;

import java.util.HashMap;
import java.util.Map;

public class TestMap {
    public int test(){
        Map<String,String> map = new HashMap<>();
        map.put("name","zhangsan");
        map.put("name","lisi");
        StringHelper.print20240503(map.get("name"));
        StringHelper.print20240503("map 的容量是：" + map.size());
        if(map.containsKey("name") && map.get("name").equals("lisi")) {
            return Result.SUCCESS;
        }else {
            return Result.FAILED;
        }
    }
}
