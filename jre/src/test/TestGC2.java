package test;

public class TestGC2 {
    public int test(){
        createObject();
        return Result.SUCCESS ;
    }

    private TestGc[] createObject(){
        TestGc[] testGcs= new TestGc[9000];
        for(int i = 0; i < 12000; i ++){
            TestGc testGc = new TestGc();
            if(i < 9000){
                testGcs[i] = testGc;
            }
        }
        return testGcs;
    }
}
