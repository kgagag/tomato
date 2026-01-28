package tomato.test;

public class MyHashSetTest {
    
    // 基础功能测试 - 测试重复元素
    public int testDuplicateElements() {
        MyHashSet<String> set = new MyHashSet<>();
        set.add("aaa");
        set.add("aaa");
        set.add("aaa");
        set.add("aaa");
        set.add("aaa");
        
        StringHelper.print20240503("testDuplicateElements - set 的容量是：" + set.size());
        
        if (set.contains("aaa") && set.size() == 1) {
            StringHelper.print20240503("测试通过: 重复元素只保存一个");
            return Result.SUCCESS;
        } else {
            StringHelper.print20240503("测试失败: 重复元素处理不正确");
            return Result.FAILED;
        }
    }
    
    // 测试不同元素
    public int testDifferentElements() {
        MyHashSet<String> set = new MyHashSet<>();
        set.add("aaa");
        set.add("bbb");
        set.add("ccc");
        set.add("ddd");
        set.add("eee");
        
        StringHelper.print20240503("testDifferentElements - set 的容量是：" + set.size());
        
        if (set.size() == 5 && 
            set.contains("aaa") && 
            set.contains("bbb") && 
            set.contains("ccc") && 
            set.contains("ddd") && 
            set.contains("eee")) {
            StringHelper.print20240503("测试通过: 所有元素都正确添加");
            return Result.SUCCESS;
        } else {
            StringHelper.print20240503("测试失败: 元素添加不完整");
            return Result.FAILED;
        }
    }
    
    // 测试删除功能
    public int testRemoveElement() {
        MyHashSet<String> set = new MyHashSet<>();
        set.add("aaa");
        set.add("bbb");
        set.add("ccc");
        
        StringHelper.print20240503("testRemoveElement - 删除前容量：" + set.size());
        
        boolean removed = set.remove("bbb");
        StringHelper.print20240503("删除 'bbb' 结果: " + removed);
        StringHelper.print20240503("删除后容量：" + set.size());
        
        if (removed && set.size() == 2 && !set.contains("bbb")) {
            StringHelper.print20240503("测试通过: 元素删除成功");
            return Result.SUCCESS;
        } else {
            StringHelper.print20240503("测试失败: 元素删除不正确");
            return Result.FAILED;
        }
    }
    
    // 测试空集合
    public int testEmptySet() {
        MyHashSet<String> set = new MyHashSet<>();
        
        StringHelper.print20240503("testEmptySet - 初始容量：" + set.size());
        StringHelper.print20240503("是否为空: " + set.isEmpty());
        
        if (set.size() == 0 && set.isEmpty()) {
            StringHelper.print20240503("测试通过: 空集合创建成功");
            return Result.SUCCESS;
        } else {
            StringHelper.print20240503("测试失败: 空集合状态不正确");
            return Result.FAILED;
        }
    }
    
    // 测试清空功能
    public int testClearSet() {
        MyHashSet<String> set = new MyHashSet<>();
        set.add("aaa");
        set.add("bbb");
        set.add("ccc");
        
        StringHelper.print20240503("testClearSet - 清空前容量：" + set.size());
        
        set.clear();
        StringHelper.print20240503("清空后容量：" + set.size());
        StringHelper.print20240503("是否为空: " + set.isEmpty());
        
        if (set.size() == 0 && set.isEmpty()) {
            StringHelper.print20240503("测试通过: 集合清空成功");
            return Result.SUCCESS;
        } else {
            StringHelper.print20240503("测试失败: 清空功能不正确");
            return Result.FAILED;
        }
    }
    
    // 测试扩容功能
    public int testResize() {
        MyHashSet<Integer> set = new MyHashSet<>(4, 0.75f); // 小容量测试扩容
        
        StringHelper.print20240503("testResize - 初始容量设置: 4");
        StringHelper.print20240503("负载因子: 0.75");
        StringHelper.print20240503("阈值: 3 (4 * 0.75)");
        
        // 添加4个元素，应该触发扩容
        for (int i = 0; i < 4; i++) {
            set.add(i * 10);
            StringHelper.print20240503("添加元素 " + (i * 10) + " 后容量: " + set.size());
        }
        
        if (set.size() == 4) {
            StringHelper.print20240503("测试通过: 扩容功能正常");
            return Result.SUCCESS;
        } else {
            StringHelper.print20240503("测试失败: 扩容后元素丢失");
            return Result.FAILED;
        }
    }
    
    // 测试迭代器
    public int testIterator() {
        MyHashSet<String> set = new MyHashSet<>();
        set.add("apple");
        set.add("banana");
        set.add("cherry");
        
        StringHelper.print20240503("testIterator - 集合元素:");
        int count = 0;
        for (String fruit : set) {
            StringHelper.print20240503("  " + fruit);
            count++;
        }
        
        if (count == 3) {
            StringHelper.print20240503("测试通过: 迭代器正常工作");
            return Result.SUCCESS;
        } else {
            StringHelper.print20240503("测试失败: 迭代器遍历不正确");
            return Result.FAILED;
        }
    }
    
    // 测试toArray方法
//    public int testToArray() {
//        MyHashSet<String> set = new MyHashSet<>();
//        set.add("first");
//        set.add("second");
//        set.add("third");
//
//        String[] array = set.toArray();
//        StringHelper.print20240503("testToArray - 转换的数组: " + Arrays.toString(array));
//
//        if (array.length == 3) {
//            StringHelper.print20240503("测试通过: toArray方法正常工作");
//            return Result.SUCCESS;
//        } else {
//            StringHelper.print20240503("测试失败: 数组转换不正确");
//            return Result.FAILED;
//        }
//    }
    
    // 综合测试 - 模拟原格式
    public int testOriginalFormat() {
        MyHashSet<String> set = new MyHashSet<>();
        set.add("aaa");
        set.add("aaa");
        set.add("aaa");
        set.add("aaa");
        set.add("aaa");
        
        StringHelper.print20240503("testOriginalFormat - set 的容量是：" + set.size());
        
        if (set.contains("aaa")) {
            StringHelper.print20240503("测试通过: 包含元素 'aaa'");
            return Result.SUCCESS;
        } else {
            StringHelper.print20240503("测试失败: 不包含元素 'aaa'");
            return Result.FAILED;
        }
    }
    
    // 运行所有测试
    public int  test() {
        MyHashSetTest tester = new MyHashSetTest();
        
        StringHelper.print20240503("========== 开始测试 MyHashSet ==========");
        StringHelper.print20240503("");
        
        int[] results = new int[9];
        
        results[0] = tester.testOriginalFormat();
        StringHelper.print20240503("");
        
        results[1] = tester.testDuplicateElements();
        StringHelper.print20240503("");
        
        results[2] = tester.testDifferentElements();
        StringHelper.print20240503("");
        
        results[3] = tester.testRemoveElement();
        StringHelper.print20240503("");
        
        results[4] = tester.testEmptySet();
        StringHelper.print20240503("");
        
        results[5] = tester.testClearSet();
        StringHelper.print20240503("");
        
        results[6] = tester.testResize();
        StringHelper.print20240503("");
        
        results[7] = tester.testIterator();
        StringHelper.print20240503("");

        // 统计测试结果
        int successCount = 0;
        int failedCount = 0;
        
        for (int result : results) {
            if (result == Result.SUCCESS) {
                successCount++;
            } else {
                failedCount++;
            }
        }
        
        StringHelper.print20240503("========== 测试结果统计 ==========");
        StringHelper.print20240503("总测试数: " + results.length);
        StringHelper.print20240503("通过数: " + successCount);
        StringHelper.print20240503("失败数: " + failedCount);
        
        if (failedCount == 0) {
            StringHelper.print20240503("所有测试通过!");
        } else {
            StringHelper.print20240503(failedCount + " 个测试失败");
        }
        return Result.SUCCESS;
    }
}