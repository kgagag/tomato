package test;

import java.util.ArrayList;
import java.util.List;

public class TestList {
    public void test(){
        List<String> stringList = new ArrayList<>();
        for(int i = 0 ; i < 10; i ++) {
            stringList.add("测试list" + i);
        }
        for(int i = 0 ; i < stringList.size();i++) {
            StringHelper.print20240503(stringList.get(i));
        }
    }

    public static void main(String[] args) {
        String[][][] strings = new String[10][][];
        int[][][][] ints = new int[1][1][0][1];
        boolean[][][][] booleans = new boolean[1][1][0][1];
        byte[][][][] bytes = new byte[1][1][0][1];
        char[][][][] chars = new char[1][1][0][1];
        short[][][][] shorts = new short[1][1][0][1];
        long[][][][] longs = new long[1][1][0][1];
        double[][][][] doubles = new double[1][1][0][1];
        float[][][][] floats = new float[1][1][0][1];

        System.out.println(strings.getClass());
        System.out.println(ints.getClass());
        System.out.println(booleans.getClass());
        System.out.println(bytes.getClass());
        System.out.println(chars.getClass());
        System.out.println(shorts.getClass());
        System.out.println(longs.getClass());
        System.out.println(doubles.getClass());
        System.out.println(floats.getClass());
        System.out.println(floats.getClass().getComponentType().getComponentType().getComponentType().getComponentType().getName());

    }
}
