package tomato.test.leetcode;

import tomato.test.Result;

public class Solution_231 {
    public boolean isPowerOfTwo(int n) {
        if(n == 1){
            return true;
        }
        if(n <= 0){
            return false;
        }
        if(n % 2 !=0){
            return false;
        }
        while ( n > 0){
           n =  n / 2;
           if(n != 1 && n % 2 != 0){
               return false;
           }
        }
        return true;
    }
    public int test(){
//        boolean flag = isPowerOfTwo(1);
        boolean flag1 = isPowerOfTwo(16);
//        boolean flag2 = isPowerOfTwo(2);
//        boolean flag4 = flag && flag1 && flag2;
        return flag1 ? Result.SUCCESS:Result.FAILED;
    }
}