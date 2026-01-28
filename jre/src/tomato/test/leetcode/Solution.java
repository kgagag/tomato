package tomato.test.leetcode;

import java.util.ArrayList;
import java.util.LinkedList;
import java.util.List;

public class Solution {
    public List<List<Integer>> levelOrder(TreeNode root) {
        List<List<Integer>> ans = new ArrayList<>();
        LinkedList<TreeNode> q = new LinkedList<>();
        q.offer(root);
        while (q.size() != 0) {
            int size = q.size();
            List<Integer> item = new ArrayList<>();
            for (int i = 0; i < size; i++) {
                TreeNode node = q.poll();
                item.add(node.val);
                if (node.left != null) {
                    q.offer(node.left);
                }
                if (node.right != null) {
                    q.offer(node.right);
                }
                if (item.size() > 0) {
                    ans.add(item);
                }
            }
        }
        return ans;
    }

    public static void main(String[] args) {
        TreeNode root = new TreeNode(3);
        Solution solution = new Solution();
        solution.levelOrder( root);
    }
}
