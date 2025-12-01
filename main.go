package main

import (
	"fmt"
	"log"
	"math/rand"
	"os"
	"sync"
	"time"
)

type Task struct {
	ID      int
	Payload string
}

func main() {
	logger := log.New(os.Stdout, "[Go DPS] ", log.LstdFlags|log.Lmicroseconds)

	const numWorkers = 4
	const numTasks = 10

	tasks := make(chan Task, numTasks)   // task queue
	results := make(chan string, numTasks)

	var wg sync.WaitGroup

	// Open result file
	resultFile, err := os.OpenFile("results_go.txt", os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	if err != nil {
		logger.Fatalf("Failed to open result file: %v", err)
	}
	defer func() {
		if err := resultFile.Close(); err != nil {
			logger.Printf("Error closing result file: %v", err)
		}
	}()

	// Start workers
	for i := 1; i <= numWorkers; i++ {
		wg.Add(1)
		go worker(i, tasks, results, &wg, logger, resultFile)
	}

	// Produce tasks
	for i := 1; i <= numTasks; i++ {
		task := Task{ID: i, Payload: fmt.Sprintf("data-%d", i)}
		logger.Printf("Main goroutine sending task %d\n", i)
		tasks <- task
	}

	close(tasks) // signal no more tasks

	// Wait for workers to finish
	wg.Wait()
	close(results)

	// Drain results (optional, mainly for demonstration)
	for res := range results {
		logger.Printf("Main received result: %s\n", res)
	}
}

func worker(id int, tasks <-chan Task, results chan<- string, wg *sync.WaitGroup, logger *log.Logger, file *os.File) {
	defer wg.Done()
	logger.Printf("Worker %d starting\n", id)
	rand.Seed(time.Now().UnixNano() + int64(id))

	for task := range tasks {
		logger.Printf("Worker %d received task %d\n", id, task.ID)
		if err := processTask(id, task, results, logger, file); err != nil {
			logger.Printf("Worker %d error processing task %d: %v\n", id, task.ID, err)
		}
	}

	logger.Printf("Worker %d finished\n", id)
}

func processTask(workerID int, task Task, results chan<- string, logger *log.Logger, file *os.File) error {
	// Simulate some computation time
	sleepMs := rand.Intn(800) + 200
	time.Sleep(time.Duration(sleepMs) * time.Millisecond)

	result := fmt.Sprintf("Worker %d processed task %d with payload %s", workerID, task.ID, task.Payload)

	// Write to file with explicit error check
	if _, err := file.WriteString(result + "\n"); err != nil {
		return fmt.Errorf("file write failed: %w", err)
	}

	// Send result back on channel
	select {
	case results <- result:
		logger.Printf("Worker %d completed task %d\n", workerID, task.ID)
	default:
		// Non-blocking send fallback – for example if nobody is reading results fast enough
		logger.Printf("Worker %d result channel full, dropping result for task %d\n", workerID, task.ID)
	}

	return nil
}