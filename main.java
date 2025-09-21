import java.util.ArrayList;
import java.util.List;

public class Main {
    static long sum(List<Integer> xs) {
        long s = 0;
        for (int v : xs) s += v;
        return s;
    }

    public static void main(String[] args) {
        for (int round = 0; round < 5; round++) {
            List<Integer> xs = new ArrayList<>();
            for (int i = 0; i < 1_000_00; i++) xs.add(i); 
            System.out.println("Java sum = " + sum(xs));
        }

        List<byte[]> hold = new ArrayList<>();
        for (int i = 0; i < 50; i++) hold.add(new byte[1_000_000]); 
        System.out.println("Holding " + hold.size() + " arrays to demonstrate retention.");
    }
}
