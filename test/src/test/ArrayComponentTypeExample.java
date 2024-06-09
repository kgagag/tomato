package test;

public class ArrayComponentTypeExample {
    public static void main(String[] args) {
        // 创建一个整数数组
        int[] intArray = new int[10];
        Class<?> intArrayClass = intArray.getClass();
        Class<?> intArrayComponentType = intArrayClass.getComponentType();
        
        // 打印结果
        System.out.println("数组类型: " + intArrayClass.getName());
        System.out.println("数组组件类型: " + intArrayComponentType.getName());

        // 创建一个字符串数组
        String[] stringArray = new String[10];
        Class<?> stringArrayClass = stringArray.getClass();
        Class<?> stringArrayComponentType = stringArrayClass.getComponentType();
        
        // 打印结果
        System.out.println("数组类型: " + stringArrayClass.getName());
        System.out.println("数组组件类型: " + stringArrayComponentType.getName());

        // 检查非数组类型
        Class<?> stringClass = String.class;
        Class<?> stringComponentType = stringClass.getComponentType();
        
        // 打印结果
        System.out.println("非数组类型: " + stringClass.getName());
        System.out.println("非数组类型组件: " + stringComponentType); // 应该为 null
    }
}
