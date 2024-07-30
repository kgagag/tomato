package test;

import java.util.HashMap;

public class Test81 {
    public int test(){
        HashMap<String,String> map = new HashMap<>();
        boolean f = map.containsKey("a");
        if(f){
            StringHelper.print20240503("aaa");
        }else {
            StringHelper.print20240503("bbb");
        }
        return Result.SUCCESS ;
    }
}
