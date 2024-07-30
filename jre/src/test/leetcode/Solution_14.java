package test.leetcode;

import test.Result;

public class Solution_14 {
    public String longestCommonPrefix(String[] strs) {
        StringBuilder sb = new StringBuilder("");
        int index = 0;
        while( true ){
            boolean flag = false;
            if(strs[0].length() <= index){
                break;
            }
            char ch = strs[0].charAt(index);
            for(int i = 1; i < strs.length;i++){
                if(index >= strs[i].length() || ch != strs[i].charAt(index)){
                    flag = true;
                    break;
                }
            }
            if( !flag ){
                sb.append(ch);
            }else{
                break;
            }
            index ++;
        }
        return sb.toString();
    }

    public int test(){
        String s = longestCommonPrefix(new String[]{"flower", "flow", "flight"});
        if(s.equals("fl")){
            return Result.SUCCESS;
        }
        return Result.FAILED;
    }
}