package test.leetcode;

import test.Result;

import java.util.HashSet;
import java.util.Set;

public class Solution_3 {
    public int lengthOfLongestSubstring(String s) {
        int n = s.length();
        Set<Character> set = new HashSet<>();
        int ans = 0;
        for (int left = 0, right = 0; right < n; ) {
            if (!set.contains(s.charAt(right))) {
                set.add(s.charAt(right));
            } else {
                while (set.contains(s.charAt(right))) {
                    set.remove(s.charAt(left++));
                }
                set.add(s.charAt(right));
            }
            right++;
            ans = Math.max(ans, right - left);
        }
        return ans;
    }

	public int test(){
		if(lengthOfLongestSubstring("abcabcadfghdfhhhhhfgfhfghfhhhfsefasfafafafafafaaafadggghkhjllxzcvzvzbb")==7){
			return Result.SUCCESS;
		}
		return Result.FAILED;
	}
}