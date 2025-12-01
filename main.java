import java.io.FileWriter;
import java.io.IOException;
import java.io.PrintWriter;
import java.util.*;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.logging.Level;
import java.util.logging.Logger;

// Represents one unit of work
class Task {
    private final int id;
    private final String payload;
    private final boolean poisonPill;

    public Task(int id, String payload) {
        this.id = id;
        this.payload = payload;
        this.poisonPill = false;
    }

    public Task(boolean poisonPill) {
        this.id = -1;
        this.payload = "";
        this.poisonPill = poisonPill;
    }

    public int getId() { return id; }
    public String getPayload() { return payload; }
    public boolean isPoisonPill() { return poisonPill; }
}

// Shared queue with synchronized add/get
class TaskQueue {
    private final Queue<Task> queue = new LinkedList<>();

    public synchronized void addTask(Task task) {
        queue.add(task);
        notifyAll(); // wake up waiting workers
    }

    public synchronized Task getTask() throws InterruptedException {
        while (queue.isEmpty()) {
            wait(); // wait until a task is available
        }
        return queue.remove();
    }
}

// Shared result store – writes to memory + file
class ResultStore implements AutoCloseable {
    private final List<String> results = new ArrayList<>();
    private final PrintWriter writer;
    private final Logger logger = Logger.getLogger(ResultStore.class.getName());

    public ResultStore(String fileName) throws IOException {
        this.writer = new PrintWriter(new FileWriter(fileName, true), true);
    }

    public synchronized void addResult(String result) {
        results.add(result);
        try {
            writer.println(result);
        } catch (Exception e) {
            logger.log(Level.SEVERE, "Error writing result to file", e);
        }
    }

    public synchronized List<String> snapshot() {
        return new ArrayList<>(results);
    }

    @Override
    public void close() {
        writer.close();
    }
}

// Worker that processes tasks from the shared queue
class Worker implements Runnable {
    private final int workerId;
    private final TaskQueue queue;
    private final ResultStore resultStore;
    private final Logger logger = Logger.getLogger(Worker.class.getName());

    public Worker(int workerId, TaskQueue queue, ResultStore resultStore) {
        this.workerId = workerId;
        this.queue = queue;
        this.resultStore = resultStore;
    }

    @Override
    public void run() {
        logger.info("Worker " + workerId + " starting");
        try {
            while (true) {
                Task task = queue.getTask(); // may block
                if (task.isPoisonPill()) {
                    logger.info("Worker " + workerId + " received poison pill, exiting");
                    break;
                }

                try {
                    processTask(task);
                } catch (Exception e) {
                    logger.log(Level.SEVERE,
                            "Worker " + workerId + " error processing task " + task.getId(), e);
                }
            }
        } catch (InterruptedException e) {
            logger.log(Level.WARNING, "Worker " + workerId + " interrupted", e);
            Thread.currentThread().interrupt();
        }
        logger.info("Worker " + workerId + " finished");
    }

    private void processTask(Task task) throws InterruptedException {
        logger.info("Worker " + workerId + " processing task " + task.getId());

        // Simulate computational delay
        Thread.sleep(200 + new Random().nextInt(800));

        String result = "Worker " + workerId + " processed task " + task.getId()
                + " with payload: " + task.getPayload();

        resultStore.addResult(result);
        logger.info("Worker " + workerId + " completed task " + task.getId());
    }
}

// Main application
public class MainApp {
    public static void main(String[] args) {
        Logger logger = Logger.getLogger(MainApp.class.getName());
        int numWorkers = 4;
        int numTasks = 10;

        TaskQueue taskQueue = new TaskQueue();
        ExecutorService executor = Executors.newFixedThreadPool(numWorkers);

        try (ResultStore resultStore = new ResultStore("results.txt")) {

            // Start workers
            for (int i = 1; i <= numWorkers; i++) {
                executor.submit(new Worker(i, taskQueue, resultStore));
            }

            // Produce tasks
            for (int i = 1; i <= numTasks; i++) {
                Task task = new Task(i, "data-" + i);
                taskQueue.addTask(task);
                logger.info("Main thread added task " + i);
            }

            // Send poison pills to stop workers
            for (int i = 0; i < numWorkers; i++) {
                taskQueue.addTask(new Task(true));
            }

        } catch (IOException e) {
            logger.log(Level.SEVERE, "Failed to initialize ResultStore", e);
        } finally {
            executor.shutdown();
        }
    }
}
