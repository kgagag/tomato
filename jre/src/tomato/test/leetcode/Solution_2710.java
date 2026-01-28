package tomato.test.leetcode;

import tomato.test.Result;

public class Solution_2710 {
    public String removeTrailingZeros(String num) {
        int l = num.length();
        while(num.charAt(l - 1) == '0'){
            l --;
        }
        return num.substring(0,l);
    }
    public int test(){
        String s = removeTrailingZeros("12300450000");
        if(s.equals("1230045")){
            return Result.SUCCESS;
        }
        return Result.FAILED;
    }
}