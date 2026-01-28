package tomato.test;

import java.util.HashMap;
import java.util.Map;

public class Code {
    /**
     *
     * 两个字符串相加
     * 别名：大数相加
     * 描述：给两个大整数, 用字符串表示, 比如”21543655", "4332656442", 都可能超过1万位, 写一个函数输出他们之和. 需要自己实现加法过程, 不能用某些语言自带的高精度加法函数.
     * 输入："21543655”, "4332656442"
     * 输出："4354200097"
     * @param number1
     * @param number2
     * @return
     */
    public String add(String number1, String number2) {
        int len1 = number1.length();
        int len2 = number2.length();
        char[] ans = new char[len1 > len2 ? len1 + 1 : len2 + 1 ];
        int i = 0;
        int left = 0;
        while (i < len1 || i < len2){
            int temp = (i < len1 ? number1.charAt(len1 - i -1) - '0' : 0) + (i < len2 ? number2.charAt(len2 - i -1) - '0' : 0 )+ left;
            left = 0;
           if (temp >= 10){
               ans[ ans.length - i - 1] = (char)(temp - 10 + '0');
               left = 1;
           }else {
               ans[ ans.length - i - 1] = (char)(temp + '0');
           }
           i++;
        }
        if (left == 1){
            ans[0] = '1';
            return new String(ans);
        }
        return new String(ans, 1, ans.length -1);
    }


    /**
     *
     * 二分查找
     * 描述：给定一个有序数组， 找某个数字， 找到返回索引， 找不到返回-1.
     * 输入：[1, 2, 3, 4, 5, 6, 7, 8, 9], 5
     * 输出：4
     * @param nums
     * @param target
     * @return
     */
    public int search(int[] nums, int target) {
        int left = 0, right = nums.length - 1;
        while (left <= right) {
            int mid = left + (right - left) / 2;
            if (nums[mid] == target) {
                return mid;
            }
            // 判断mid在左边有序部分还是右边有序部分
            if (nums[mid] >= nums[left]) {
                if (target >= nums[left] && target < nums[mid]) {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            } else {
                if (target > nums[mid] && target <= nums[right]) {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
        }
        return -1; // 没找到
    }

    /**
     *
     * 数组中只出现一次的数字
     * 描述：给一个长度为n的数组， 数组中只有两个数字出现了一次， 其他的数字都出现了两次， 请找出这两个只出现一次的数字。
     * 输入：[1, 2, 3, 2, 1, 4, 5, 5, 4]
     * 输出：3, 4
     * @param nums
     * @return
     */
    public int findSingleNumber(int[] nums) {
        Map<Integer, Integer> map = new HashMap<>();
        for (int num : nums) {
            map.put(num, map.getOrDefault(num, 0) + 1);
        }
        for (Map.Entry<Integer, Integer> entry : map.entrySet()) {
            if (entry.getValue() == 1) {
                return entry.getKey();
            }
        }
        return -1;
    }
}
