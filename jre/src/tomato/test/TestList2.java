package tomato.test;

import java.util.ArrayList;
import java.util.List;

public class TestList2 {
    public int test(){
        List<String> stringList = new ArrayList<>();
        for(int i = 0 ; i < 100; i ++) {
            stringList.add("测试list" + i);
        }
        int i = stringList.size();
        while (i > 10){
            stringList.remove(0);
            i --;
        }
        StringHelper.print20240503("stringList remove 后的容量是 ：" +stringList.size());

        if(stringList.size() == 10){
            return Result.SUCCESS;
        }
        return Result.FAILED;
    }
}
