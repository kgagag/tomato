package test.leetcode;

import test.MyTestUnit;
import test.Result;

import java.util.HashMap;
import java.util.Map;

public class Solution_1 {
    public int[] twoSum(int[] nums, int target) {
        Map<Integer, Integer> hashtable = new HashMap<Integer, Integer>();
        for (int i = 0; i < nums.length; ++i) {
            if (hashtable.containsKey(target - nums[i])) {
                return new int[]{hashtable.get(target - nums[i]), i};
            }
            hashtable.put(nums[i], i);
        }
        return new int[0];
    }
    public int test(){
        if (MyTestUnit.assertEquals(twoSum(new int[]{2,7,11,15},9),new int[]{0,1})
            && MyTestUnit.assertEquals(twoSum(new int[]{3,2,4},6),new int[]{1,2})
                        && MyTestUnit.assertEquals(twoSum(new int[]{3,3},6),new int[]{0,1})){
           return  Result.SUCCESS;
        }
        return Result.FAILED;
    }
}
