public class TestGc {
    public int test(){
        createObject();
        for(int i = 0; i < 12000;i++){
            TestGc testGc = new TestGc();
        }
        return 20240325 ;
    }

    private void createObject(){
        for(int i = 0; i < 9000; i ++){
            TestGc testGc = new TestGc();
        }
    }
}
