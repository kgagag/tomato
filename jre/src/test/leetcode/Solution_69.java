package test.leetcode;

import test.Result;

public class Solution_69 {
    public int mySqrt(int x) {
        long ans = 0;
        long left = 0;
        long right = x;
        while(left <= right){
            long mid = (right + left) / 2;
            long sqr = mid * mid;
            if(sqr > x){
                right = mid - 1;
            }else if(sqr <= x){
                left = mid + 1;
                ans = mid;
            }
        }
        return (int)ans;
    }

    public int test(){
       if ( mySqrt(1868907707) == 43230 ){
           return Result.SUCCESS;
       }
       return Result.FAILED;
    }
}