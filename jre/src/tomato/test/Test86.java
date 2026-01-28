package tomato.test;

public class Test86 {
    public void  test(){
        String[][][] strings = new String[10][1][1];
        int[][][][] ints = new int[1][1][0][1];
        boolean[][][][] booleans = new boolean[1][1][0][1];
        byte[]  bytes = new byte[1];
        char[][][][] chars = new char[1][1][0][1];
        short[][][][] shorts = new short[1][1][0][1];
        long[][][][] longs = new long[1][1][0][1];
        double[][][][] doubles = new double[1][1][0][1];
        float[][][][] floats = new float[1][1][0][1];

         StringHelper.print20240503(strings.getClass().getComponentType().getName());
        StringHelper.print20240503(strings.getClass().getComponentType().getComponentType().getName());

        StringHelper.print20240503(ints.getClass().getComponentType().getName());
        StringHelper.print20240503(booleans.getClass().getComponentType().getName());
        StringHelper.print20240503(bytes.getClass().getComponentType().getName());
        StringHelper.print20240503(chars.getClass().getComponentType().getName());
        StringHelper.print20240503(shorts.getClass().getComponentType().getName());
        StringHelper.print20240503(longs.getClass().getComponentType().getName());
        StringHelper.print20240503(doubles.getClass().getComponentType().getName());
        StringHelper.print20240503(floats.getClass().getComponentType().getName());
        StringHelper.print20240503(floats.getClass().getComponentType().getComponentType().getComponentType().getName());

    }

    public static void main(String[] args) {
        new Test86().test();
    }
}
