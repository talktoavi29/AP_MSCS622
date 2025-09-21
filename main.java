import java.util.ArrayList;
import java.util.List;

public class Main {
    static long usedMB() {
        Runtime rt = Runtime.getRuntime();
        return (rt.totalMemory() - rt.freeMemory()) / (1024 * 1024);
    }

    public static void main(String[] args) {
        System.out.println("Used MB (start): " + usedMB());

        for (int round = 0; round < 3; round++) {
            List<Integer> xs = new ArrayList<>();
            for (int i = 0; i < 100_000; i++) xs.add(i);
            System.out.println("Round " + round + " sum=" + xs.stream().mapToInt(Integer::intValue).sum());
        }
        System.out.println("Used MB (after short-lived): " + usedMB());

        List<byte[]> hold = new ArrayList<>();
        for (int i = 0; i < 20; i++) hold.add(new byte[1_000_000]); 
        System.out.println("Used MB (after hold): " + usedMB());

        hold = null;
        System.gc(); 
        System.out.println("Used MB (after GC hint): " + usedMB());
    }
}
