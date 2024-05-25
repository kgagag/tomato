package test;

import java.util.HashMap;
import java.util.Map;

public class Test81 {
    public void test(){
        HashMap<String,String> map = new HashMap<>();
        boolean f = map.containsKey("a");
        if(f){
            StringHelper.print20240503("aaa");
        }else {
            StringHelper.print20240503("bbb");
        }

    }
}
