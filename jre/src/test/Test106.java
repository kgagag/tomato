package test;

import java.util.HashMap;

public class Test106 {

    public int test() {
       String a = "a" ;
       String b = null;
       if(a == b){
           return Result.FAILED ;
       }else {
           return Result.SUCCESS ;
       }
    }
    public static void main(String[] args) {
        Test106 test106 = new Test106();
        test106.test();
    }
}
