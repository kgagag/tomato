package tomato.test;

import java.util.HashSet;
import java.util.Set;

public class TestSet {
    public int test(){
        Set<String> set = new HashSet<>();
        set.add("aaa");
        set.add("aaa");
        set.add("aaa");
        set.add("aaa");
        set.add("aaa");
        StringHelper.print20240503("set 的容量是：" + set.size());
        if(set.contains("aaa")){
            return Result.SUCCESS;
        }else {
            return Result.FAILED;
        }
    }
}
